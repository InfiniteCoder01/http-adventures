window.addEventListener("keydown", event => {
    if (event.code == "F3") {
        drawDebugInfo = !drawDebugInfo;
        event.preventDefault();
    }
});

canvas.addEventListener("mousedown", event => {
    const [x, y] = cameraToWorld(event.offsetX, event.offsetY);
    const objects = Object.values(world.objects)
    objects.sort((a, b) => b.y - a.y);
    for (const object of objects) {
        const [w, h] = object.size();
        if (x > object.x + 0.5 - w / 2 && x < object.x + 0.5 + w / 2 && y > object.y + 0.5 - h && y < object.y + 0.5) {
            player.pathfind(object.x, object.y);
            return;
        }
    }

    player.pathfind(Math.floor(x), Math.floor(y));
});

socket.addEventListener("open", () => setInterval(() => {
    if (!player) return;
    world.update();
    [camera.x, camera.y] = [player.x * world.tileSize, player.y * world.tileSize];
}, 1000 / 60));
