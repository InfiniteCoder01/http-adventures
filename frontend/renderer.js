let drawDebugInfo = false;
const camera = {
    x: 0,
    y: 0,
};

const textures = {};
function drawTile(texture, tile, x, y) {
    const image = textures[texture];
    if (!image) return;
    ctx.drawImage(image, tile[0] * 32, tile[1] * 32, 32, 32, x, y, 32, 32);
}

function drawChunk(cx, cy) {
    const chunk = world.get(cx, cy);
    if (!chunk) return;
    for (const layer of chunk.layers) {
        for (let tx = 0; tx < world.chunkSize; tx++) {
            for (let ty = 0; ty < world.chunkSize; ty++) {
                const tile = layer[tx + ty * world.chunkSize];
                if (tile === null) continue;
                drawTile(
                    "spritesheet.png",
                    [tile % 16, Math.floor(tile / 16)],
                    (cx * world.chunkSize + tx) * world.tileSize,
                    (cy * world.chunkSize + ty) * world.tileSize,
                );
            }
        }
    }

    if (drawDebugInfo) {
        ctx.strokeStyle = "red";
        ctx.strokeRect(
            cx * world.chunkSize * world.tileSize,
            cy * world.chunkSize * world.tileSize,
            world.chunkSize * world.tileSize,
            world.chunkSize * world.tileSize
        );
    }
}

function renderFrame() {
    const minSize = 512;
    const scale = Math.round(Math.min(canvas.width, canvas.height) / minSize);
    const width = canvas.width / scale, height = canvas.height / scale;
    ctx.resetTransform();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.scale(scale, scale);
    ctx.translate(Math.round(-camera.x + width / 2), Math.round(-camera.y + height / 2));

    const min = c => Math.floor(c / world.tileSize / world.chunkSize);
    const max = c => Math.ceil(c / world.tileSize / world.chunkSize);
    for (let cy = min(camera.y - height / 2); cy < max(camera.y + height / 2); cy++) {
        for (let cx = min(camera.x - width / 2); cx < max(camera.x + width / 2); cx++) {
            drawChunk(cx, cy);
        }
    }
    ctx.fillRect(camera.x - 5, camera.y - 5, 10, 10);

    requestAnimationFrame(renderFrame);
}

requestAnimationFrame(renderFrame);
