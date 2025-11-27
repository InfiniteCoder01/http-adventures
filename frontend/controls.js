const keys = {};

onkeydown = event => keys[event.code] = true;
onkeyup = event => keys[event.code] = false;
const key = name => keys[name] || false;

setInterval(() => {
    camera.x += (key('KeyD') - key('KeyA')) * 1;
    camera.y += (key('KeyS') - key('KeyW')) * 1;
}, 1000 / 60);
