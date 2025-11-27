const camera = {
    x: 0,
    y: 0,
    zoom: 5,
};

setInterval(() => {
    const scale = Math.min(canvas.width, canvas.height) / 512;
    ctx.resetTransform();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.scale(scale, scale);
    ctx.translate(-camera.x, -camera.y);

    if (textures['test.png']) ctx.drawImage(textures['test.png'], 0, 0);
}, 1000 / 60);
