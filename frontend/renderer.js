let drawDebugInfo = false;
const camera = {
    x: 0,
    y: 0,
};

const textures = {};
function texture(id) {
    if (!textures[id]) {
        textures[id] = new Image();
        textures[id].src = `${document.location.origin}/assets/${id}`;
    }
    return textures[id];
}

function drawTile(image, tile, tileSize, x, y) {
    if (typeof tileSize === "number") tileSize = [tileSize, tileSize];
    if (typeof image === "string") image = texture(image);
    ctx.drawImage(
        image,
        tile[0] * tileSize[0],
        tile[1] * tileSize[1],
        ...tileSize, x, y, ...tileSize);
}

function drawChunk(cx, cy) {
    const chunk = world.get(cx, cy);
    if (!chunk) return;
    for (const layer of chunk.layers) {
        for (let tx = 0; tx < world.chunkSize; tx++) {
            for (let ty = 0; ty < world.chunkSize; ty++) {
                const tile = layer[tx + ty * world.chunkSize];
                if (tile === null) continue;
                const row = world.tileset.width / world.tileSize;
                drawTile(
                    world.tileset,
                    [tile % row, Math.floor(tile / row)],
                    world.tileSize,
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

function drawObject(object) {
    const image = texture(object.texture);
    ctx.drawImage(image, object.x - image.width / 2, object.y - image.height);
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

    const objects = Object.values(world.objects)
    objects.sort((a, b) => a.y - b.y);
    for (const object of objects) drawObject(object);

    requestAnimationFrame(renderFrame);
}

requestAnimationFrame(renderFrame);
