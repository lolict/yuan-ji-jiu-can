package com.mqf.parasite

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.accounts.AbstractAccountAuthenticator
import android.accounts.AccountAuthenticatorResponse
import android.accounts.Account
import android.accounts.NetworkErrorException
import android.os.Bundle

class MqfAuthenticatorService : Service() {
    private lateinit var authenticator: MqfAuthenticator
    override fun onCreate() { authenticator = MqfAuthenticator(this) }
    override fun onBind(intent: Intent?): IBinder? = authenticator.iBinder
}

class MqfAuthenticator(context: Context) : AbstractAccountAuthenticator(context) {
    override fun editProperties(r: AccountAuthenticatorResponse?, s: String?): Bundle? = null
    override fun addAccount(r: AccountAuthenticatorResponse?, s: String?, s2: String?,
        strings: Array<out String>?, bundle: Bundle?): Bundle? = null
    override fun confirmCredentials(r: AccountAuthenticatorResponse?, a: Account?, b: Bundle?): Bundle? = null
    override fun getAuthToken(r: AccountAuthenticatorResponse?, a: Account?, s: String?,
        b: Bundle?): Bundle? = null
    override fun getAuthTokenLabel(s: String?): String? = null
    override fun updateCredentials(r: AccountAuthenticatorResponse?, a: Account?, s: String?,
        b: Bundle?): Bundle? = null
    override fun hasFeatures(r: AccountAuthenticatorResponse?, a: Account?,
        strings: Array<out String>?): Bundle? = null
    override fun getAccountRemovalAllowed(r: AccountAuthenticatorResponse?, a: Account?): Bundle? = null
}
