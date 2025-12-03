const keys = {};

const key = name => keys[name] || false;
onkeyup = event => keys[event.code] = false;
onkeydown = event => {
    keys[event.code] = true;
    if (event.code == "F3") {
        drawDebugInfo = !drawDebugInfo;
        event.preventDefault();
    }
}

socket.addEventListener("open", () => setInterval(() => {
    if (!player) return;
    player.x += (key('KeyD') - key('KeyA')) * 1;
    player.y += (key('KeyS') - key('KeyW')) * 1;
    [camera.x, camera.y] = [player.x, player.y];
    sendPlayerUpdate();
}, 1000 / 60));
