package com.example.linkup.core.data.native

import android.util.Log

object NativeLibLoader {
    private const val TAG = "NativeLibLoader"
    private var isLoaded = false

    fun loadNativeLibrary() {
        if (!isLoaded) {
            try {
                System.loadLibrary("linkup_ffi")
                isLoaded = true
                Log.i(TAG, "Successfully loaded liblinkup_ffi.so native library")
            } catch (e: UnsatisfiedLinkError) {
                Log.w(TAG, "liblinkup_ffi.so not found in APK native libs, falling back to simulated runtime mode")
            }
        }
    }
}
