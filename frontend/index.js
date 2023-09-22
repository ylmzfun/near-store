import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

// Setup on page load
window.onload = async () => {
  let isSignedIn = await wallet.startUp();

  if (isSignedIn) {
    signedInFlow();
    fetchOrderList();
  } else {
    signedOutFlow();
  }

};

// Button clicks
document.querySelector('#buy1').onclick = doUserAction;
document.querySelector('#buy2').onclick = doUserAction;
document.querySelector('#buy3').onclick = doUserAction;
document.querySelector('#sign-in-button').onclick = () => { wallet.signIn(); };
document.querySelector('#sign-out-button').onclick = () => { wallet.signOut(); };

// Take the new greeting and send it to the contract
async function doUserAction(event) {
  event.preventDefault();
  let num = event.target.previousElementSibling;
  let name = event.target.dataset.name
  let price = event.target.dataset.price
  console.log(name)
  console.log(price)
  await wallet.callMethod({ method: 'set_shoppingcar', args: {
    num: parseInt(num.value),text: name ,price: parseInt(price) }, contractId: CONTRACT_ADDRESS });

  // ===== Fetch the data from the blockchain =====
  await fetchOrderList();
}

// Get greeting from the contract on chain
async function fetchOrderList() {
  const from = 0
  const orderList = await wallet.viewMethod({ method: 'get_shoppingcar',
    contractId: CONTRACT_ADDRESS,args: { from: from,limit:100 } });
  if (orderList.length > 0) {
    document.querySelector("#order-list").style.display = "block"
    let content = ""
    for(let i=0;i<orderList.length; i++){
      content += "<li>"
      content += orderList[i].sender  + "购买了" + orderList[i].num+"只" + orderList[i].text
      content += "</li>"
    }
    document.querySelector("#order-list-list").innerHTML  = content
  }else {
    document.querySelector("#order-list").style.display = "none"
  }
}

// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
}

// Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = wallet.accountId;
  });
}