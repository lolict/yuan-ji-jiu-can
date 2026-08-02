package com.mqf.parasite

import android.content.ContentProvider
import android.content.ContentValues
import android.database.Cursor
import android.net.Uri

class MqfStubProvider : ContentProvider() {
    override fun onCreate(): Boolean = true
    override fun query(u: Uri, p: Array<out String>?, s: String?,
        sp: Array<out String>?, o: String?): Cursor? = null
    override fun getType(u: Uri): String? = null
    override fun insert(u: Uri, v: ContentValues?): Uri? = null
    override fun delete(u: Uri, s: String?, sp: Array<out String>?): Int = 0
    override fun update(u: Uri, v: ContentValues?, s: String?,
        sp: Array<out String>?): Int = 0
}
