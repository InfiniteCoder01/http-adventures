window.addEventListener("keydown", event => {
    if (event.code == "F3") {
        drawDebugInfo = !drawDebugInfo;
        event.preventDefault();
    } else if (event.code == "KeyE") {
        if (currentUI) currentUI = null;
        else currentUI = new InventoryUI();
    }
});

canvas.addEventListener("mousedown", event => {
    if (currentUI) {
        currentUI.click(event);
        return;
    }

    const plr = world.obj(player);
    if (!plr) return;

    const [x, y] = cameraToWorld(event.offsetX, event.offsetY);

    const objects = Object.entries(world.objects)
    objects.sort((a, b) => b[1].y - a[1].y);
    for (const [id, object] of objects) {
        const [w, h] = object.size();
        if (x > object.x + 0.5 - w / 2 && x < object.x + 0.5 + w / 2 && y > object.y + 0.5 - h && y < object.y + 0.5) {
            plr.target = id;
            return;
        }
    }

    plr.target = [Math.floor(x), Math.floor(y)];
});

socket.addEventListener("open", () => setInterval(() => {
    world.update();
    const plr = world.obj(player);
    if (plr) {
        [camera.x, camera.y] = [plr.x * world.tileSize, plr.y * world.tileSize];
    }
}, 1000 / 60));
