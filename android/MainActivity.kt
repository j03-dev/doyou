@file:OptIn(androidx.media3.common.util.UnstableApi::class)

package dev.dioxus.main

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import androidx.core.app.ServiceCompat
import androidx.core.content.ContextCompat
import androidx.media3.common.AudioAttributes
import androidx.media3.common.C
import androidx.media3.common.ForwardingPlayer
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaMetadata
import androidx.media3.common.PlaybackException
import androidx.media3.common.Player
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.session.MediaSession

typealias BuildConfig = com.example.Doyou.BuildConfig

private const val CHANNEL_ID = "doyou_playback"
private const val NOTIFICATION_ID = 1
private const val ACTION_PREV = "dev.dioxus.main.PREV"
private const val ACTION_TOGGLE = "dev.dioxus.main.TOGGLE"
private const val ACTION_NEXT = "dev.dioxus.main.NEXT"
private const val EXTRA_URL = "dev.dioxus.main.URL"
private const val EXTRA_TITLE = "dev.dioxus.main.TITLE"
private const val EXTRA_ARTIST = "dev.dioxus.main.ARTIST"

class MainActivity : WryActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        appContext = applicationContext
    }

    companion object {
        var appContext: Context? = null
            private set
        var trackTitle = ""
        var trackArtist = ""

        private val mainHandler = Handler(Looper.getMainLooper())

        @JvmStatic
        fun playTrack(url: String, title: String, artist: String) {
            trackTitle = title
            trackArtist = artist
            mainHandler.post {
                val context = appContext ?: return@post
                val intent = Intent(context, DioxusForegroundService::class.java)
                    .putExtra(EXTRA_URL, url)
                    .putExtra(EXTRA_TITLE, title)
                    .putExtra(EXTRA_ARTIST, artist)
                ContextCompat.startForegroundService(context, intent)
            }
        }

        @JvmStatic
        fun pausePlayback() {
            mainHandler.post { DioxusForegroundService.instance?.pausePlayback() }
        }

        @JvmStatic
        fun resumePlayback() {
            mainHandler.post { DioxusForegroundService.instance?.resumePlayback() }
        }

        @JvmStatic
        fun stopPlayback() {
            mainHandler.post { DioxusForegroundService.instance?.requestStop() }
        }

        @JvmStatic
        private external fun onPlayerEvent(event: String)

        fun emit(event: String) {
            try {
                onPlayerEvent(event)
            } catch (err: Throwable) {
                err.printStackTrace()
            }
        }
    }
}

class DioxusForegroundService : Service() {
    private var player: ExoPlayer? = null
    private var mediaSession: MediaSession? = null
    private var stopped = false
    private val mainHandler = Handler(Looper.getMainLooper())

    private val progressTask = object : Runnable {
        override fun run() {
            val current = player ?: return
            if (stopped || !current.isPlaying) return
            val duration = if (current.duration > 0) current.duration / 1000 else -1L
            MainActivity.emit("progress:${current.currentPosition / 1000}:$duration")
            mainHandler.postDelayed(this, 1000)
        }
    }

    override fun onCreate() {
        super.onCreate()
        instance = this
        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val url = intent?.getStringExtra(EXTRA_URL)
        if (!url.isNullOrBlank()) {
            stopped = false
            MainActivity.trackTitle = intent.getStringExtra(EXTRA_TITLE).orEmpty()
            MainActivity.trackArtist = intent.getStringExtra(EXTRA_ARTIST).orEmpty()
            ServiceCompat.startForeground(
                this,
                NOTIFICATION_ID,
                buildNotification(),
                ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK,
            )
            startPlayback(url, MainActivity.trackTitle, MainActivity.trackArtist)
            updateNotification()
        } else if (!stopped) {
            when (intent?.action) {
                ACTION_PREV -> MainActivity.emit("prev")
                ACTION_NEXT -> MainActivity.emit("next")
                ACTION_TOGGLE -> {
                    val current = player
                    if (current != null && current.isPlaying) current.pause() else current?.play()
                }
            }
            ServiceCompat.startForeground(
                this,
                NOTIFICATION_ID,
                buildNotification(),
                ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PLAYBACK,
            )
            updateNotification()
        }
        return START_NOT_STICKY
    }

    override fun onDestroy() {
        stopProgressUpdates()
        mediaSession?.release()
        mediaSession = null
        player?.release()
        player = null
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.cancel(NOTIFICATION_ID)
        instance = null
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = null

    fun pausePlayback() {
        player?.pause()
    }

    fun resumePlayback() {
        player?.play()
    }

    @Suppress("DEPRECATION")
    fun requestStop() {
        if (stopped) return
        stopped = true
        stopProgressUpdates()
        player?.stop()
        mediaSession?.release()
        mediaSession = null
        stopForeground(true)
        stopSelf()
    }

    private fun startPlayback(url: String, title: String, artist: String) {
        ensurePlayer()
        val current = player ?: return
        current.setMediaItem(
            MediaItem.Builder()
                .setUri(url)
                .setMediaMetadata(
                    MediaMetadata.Builder()
                        .setTitle(title)
                        .setArtist(artist)
                        .build(),
                )
                .build(),
        )
        current.prepare()
        current.play()
    }

    private fun ensurePlayer() {
        if (player != null) return
        val audioAttributes = AudioAttributes.Builder()
            .setUsage(C.USAGE_MEDIA)
            .setContentType(C.AUDIO_CONTENT_TYPE_MUSIC)
            .build()
        val exoPlayer = ExoPlayer.Builder(this)
            .setAudioAttributes(audioAttributes, true)
            .build()
        player = exoPlayer
        val sessionPlayer = object : ForwardingPlayer(exoPlayer) {
            override fun hasNextMediaItem(): Boolean = true
            override fun hasPreviousMediaItem(): Boolean = true
            override fun getNextMediaItemIndex(): Int = 0
            override fun getPreviousMediaItemIndex(): Int = 0
            override fun seekToNext() {
                MainActivity.emit("next")
            }

            override fun seekToNextMediaItem() {
                MainActivity.emit("next")
            }

            override fun seekToPrevious() {
                MainActivity.emit("prev")
            }

            override fun seekToPreviousMediaItem() {
                MainActivity.emit("prev")
            }
        }
        mediaSession = MediaSession.Builder(this, sessionPlayer).build()
        exoPlayer.addListener(
            object : Player.Listener {
                override fun onIsPlayingChanged(isPlaying: Boolean) {
                    if (stopped) return
                    MainActivity.emit("state:${if (isPlaying) 1 else 0}")
                    if (isPlaying) startProgressUpdates() else stopProgressUpdates()
                    updateNotification()
                }

                override fun onPlaybackStateChanged(playbackState: Int) {
                    if (stopped) return
                    if (playbackState == Player.STATE_ENDED) MainActivity.emit("ended")
                    updateNotification()
                }

                override fun onPlayerError(error: PlaybackException) {
                    if (stopped) return
                    MainActivity.emit("error:${error.errorCode}")
                    stopProgressUpdates()
                    updateNotification()
                }
            },
        )
    }

    private fun startProgressUpdates() {
        mainHandler.removeCallbacks(progressTask)
        mainHandler.post(progressTask)
    }

    private fun stopProgressUpdates() {
        mainHandler.removeCallbacks(progressTask)
    }

    private fun updateNotification() {
        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.notify(NOTIFICATION_ID, buildNotification())
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
        val playing = player?.isPlaying == true
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
        val session = mediaSession
        if (session != null) {
            builder.setStyle(
                Notification.MediaStyle()
                    .setMediaSession(session.platformToken)
                    .setShowActionsInCompactView(0, 1, 2),
            )
        }
        return builder.build()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel =
                NotificationChannel(CHANNEL_ID, "Playback", NotificationManager.IMPORTANCE_LOW)
            channel.setShowBadge(false)
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }
    }

    companion object {
        var instance: DioxusForegroundService? = null
            private set
    }
}
