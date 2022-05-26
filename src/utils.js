import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig('testnet')

export async function initContract() {
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  window.walletConnection = new WalletConnection(near)

  window.accountId = window.walletConnection.getAccountId()

  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {

    viewMethods: ['get_greeting'],

    changeMethods: ['set_greeting'],
  })
}

export function logout() {
  window.walletConnection.signOut()

  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}
