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
    camera.x += (key('KeyD') - key('KeyA')) * 1;
    camera.y += (key('KeyS') - key('KeyW')) * 1;
    sendPlayerUpdate();
}, 1000 / 60));
