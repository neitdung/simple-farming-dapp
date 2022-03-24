import { connect, Contract, keyStores, WalletConnection } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig(process.env.NODE_ENV || 'development')

export async function initContract() {
  window.Buffer = window.Buffer || require('buffer').Buffer;
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))
  window.walletConnection = new WalletConnection(near)

  window.accountId = window.walletConnection.getAccountId()

  window.farmingContract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    viewMethods: ['get_number_of_farms', 'list_farms', 'list_seeds'],
    changeMethods: ['stake', 'withdraw', 'create_farm', 'claim_reward_by_farm', 'get_call', 'ft_deposit'],
  })
  window.ftContract = []
  window.farmingContract.list_seeds({})
    .then(seeds => {
      if (seeds) {
        if (seeds.length) {
          seeds.forEach(seedId => {
            let newFtContract = new Contract(window.walletConnection.account(), seedId, {
              viewMethods: ['ft_metadata', 'ft_balance_of', 'storage_balance_of', 'ft_total_supply'],
              changeMethods: ['ft_transfer', 'ft_transfer_call', 'storage_deposit'],
            })
            window.ftContract.push(newFtContract)
          })
        }
      }
    })
}

export function logout() {
  window.walletConnection.signOut()
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login() {
  window.walletConnection.requestSignIn(nearConfig.contractName)
}
