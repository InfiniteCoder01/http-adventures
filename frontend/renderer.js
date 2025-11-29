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
    // console.log(world.chunks);
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
}

function renderFrame() {
    const minSize = 512;
    const scale = Math.round(Math.min(canvas.width, canvas.height) / minSize);
    const width = canvas.width / scale, height = canvas.height / scale;
    ctx.resetTransform();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.scale(scale, scale);
    ctx.translate(-camera.x, -camera.y);

    const min = c => Math.floor(c / world.tileSize / world.chunkSize);
    const max = c => Math.ceil(c / world.tileSize / world.chunkSize);
    for (let cy = min(camera.y); cy < max(camera.y + height); cy++) {
        for (let cx = min(camera.x); cx < max(camera.x + width); cx++) {
            drawChunk(cx, cy);
        }
    }

    requestAnimationFrame(renderFrame);
}

requestAnimationFrame(renderFrame);
