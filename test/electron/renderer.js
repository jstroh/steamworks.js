/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(2957110);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName

document.getElementById('activateOverlay').addEventListener('click', function() {
    client.overlay.activateToWebPage('https://www.example.com/')
})

client.callback.register(steamworks.SteamCallback.SteamInventoryFullUpdate, (result) => {
    const items = client.inventory.getResultItems(result.handle);
    console.log(items);
});

setInterval(() => {
    client.inventory.getAllItems();
}, 1000)