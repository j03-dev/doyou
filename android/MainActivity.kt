package dev.dioxus.main

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.media.MediaMetadata
import android.media.session.MediaSession
import android.media.session.PlaybackState
import android.os.Build
import android.os.Bundle
import android.os.IBinder
import android.os.PowerManager
import android.webkit.WebView
import androidx.core.app.ServiceCompat
import androidx.core.content.ContextCompat

typealias BuildConfig = com.example.Doyou.BuildConfig

private const val CHANNEL_ID = "doyou_playback"
private const val NOTIFICATION_ID = 1
private const val ACTION_PREV = "dev.dioxus.main.PREV"
private const val ACTION_TOGGLE = "dev.dioxus.main.TOGGLE"
private const val ACTION_NEXT = "dev.dioxus.main.NEXT"

class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        instance = this
    }

    override fun onWebViewCreate(webView: WebView) {
        webViewRef = webView
    }

    override fun onDestroy() {
        stopService(Intent(this, DioxusForegroundService::class.java))
        instance = null
        webViewRef = null
        super.onDestroy()
    }

    companion object {
        var instance: MainActivity? = null
            private set
        var webViewRef: WebView? = null
            private set
        var serviceRunning = false
        var trackTitle = ""
        var trackArtist = ""
        var isPlaying = false

        @JvmStatic
        fun startOrUpdatePlayback(title: String, artist: String, playing: Boolean) {
            trackTitle = title
            trackArtist = artist
            isPlaying = playing
            val activity = instance ?: return
            val service = DioxusForegroundService.instance
            if (serviceRunning && service != null) {
                service.refresh()
            } else {
                serviceRunning = true
                ContextCompat.startForegroundService(
                    activity,
                    Intent(activity, DioxusForegroundService::class.java),
                )
            }
        }
    }
}

class DioxusForegroundService : Service() {
    private var mediaSession: MediaSession? = null
    private var wakeLock: PowerManager.WakeLock? = null

    override fun onCreate() {
        super.onCreate()
        instance = this
        createNotificationChannel()
        mediaSession = MediaSession(this, "doyou_playback").apply {
            setCallback(SessionCallback())
            setPlaybackState(playbackState())
            isActive = true
        }
        wakeLock = (getSystemService(Context.POWER_SERVICE) as PowerManager)
            .newWakeLock(PowerManager.PARTIAL_WAKE_LOCK, "doyou:playback").apply {
                setReferenceCounted(false)
                acquire()
            }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_PREV -> dispatch("prev")
            ACTION_TOGGLE -> dispatch("playpause")
            ACTION_NEXT -> dispatch("next")
        }
        ServiceCompat.startForeground(
            this,
            NOTIFICATION_ID,
            buildNotification(),
            ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK,
        )
        refresh()
        return START_NOT_STICKY
    }

    override fun onDestroy() {
        instance = null
        MainActivity.serviceRunning = false
        wakeLock?.let { if (it.isHeld) it.release() }
        wakeLock = null
        mediaSession?.isActive = false
        mediaSession?.release()
        mediaSession = null
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    fun refresh() {
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.notify(NOTIFICATION_ID, buildNotification())
        mediaSession?.setMetadata(
            MediaMetadata.Builder()
                .putString(MediaMetadata.METADATA_KEY_TITLE, MainActivity.trackTitle)
                .putString(MediaMetadata.METADATA_KEY_ARTIST, MainActivity.trackArtist)
                .build(),
        )
        mediaSession?.setPlaybackState(playbackState())
    }

    private fun playbackState(): PlaybackState {
        val actions = PlaybackState.ACTION_PLAY or
            PlaybackState.ACTION_PAUSE or
            PlaybackState.ACTION_PLAY_PAUSE or
            PlaybackState.ACTION_SKIP_TO_NEXT or
            PlaybackState.ACTION_SKIP_TO_PREVIOUS
        val state = if (MainActivity.isPlaying) {
            PlaybackState.STATE_PLAYING
        } else {
            PlaybackState.STATE_PAUSED
        }
        return PlaybackState.Builder()
            .setActions(actions)
            .setState(state, 0, 1f)
            .build()
    }

    private fun buildNotification(): Notification {
        val contentIntent = PendingIntent.getActivity(
            this,
            0,
            Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
        )
        fun action(command: String, icon: Int, label: String): Notification.Action {
            val intent = PendingIntent.getService(
                this,
                command.hashCode(),
                Intent(this, DioxusForegroundService::class.java).setAction(command),
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
            )
            return Notification.Action.Builder(icon, label, intent).build()
        }
        val playing = MainActivity.isPlaying
        val builder = Notification.Builder(this, CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_media_play)
            .setContentTitle(MainActivity.trackTitle)
            .setContentText(MainActivity.trackArtist)
            .setContentIntent(contentIntent)
            .setVisibility(Notification.VISIBILITY_PUBLIC)
            .setShowWhen(false)
            .setOngoing(playing)
            .addAction(action(ACTION_PREV, android.R.drawable.ic_media_previous, "Previous"))
            .addAction(
                action(
                    ACTION_TOGGLE,
                    if (playing) android.R.drawable.ic_media_pause else android.R.drawable.ic_media_play,
                    if (playing) "Pause" else "Play",
                ),
            )
            .addAction(action(ACTION_NEXT, android.R.drawable.ic_media_next, "Next"))
        val token = mediaSession?.sessionToken
        if (token != null) {
            builder.setStyle(
                Notification.MediaStyle()
                    .setMediaSession(token)
                    .setShowActionsInCompactView(0, 1, 2),
            )
        }
        return builder.build()
    }

    private fun dispatch(command: String) {
        val script =
            "window.dispatchEvent(new CustomEvent('doyou-command',{detail:'$command'}))"
        MainActivity.webViewRef?.evaluateJavascript(script, null)
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel =
                NotificationChannel(CHANNEL_ID, "Playback", NotificationManager.IMPORTANCE_LOW)
            channel.setShowBadge(false)
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }
    }

    private inner class SessionCallback : MediaSession.Callback() {
        override fun onPlay() = dispatch("play")
        override fun onPause() = dispatch("pause")
        override fun onStop() = dispatch("pause")
        override fun onSkipToNext() = dispatch("next")
        override fun onSkipToPrevious() = dispatch("prev")
    }

    companion object {
        var instance: DioxusForegroundService? = null
            private set
    }
}
