class BaseUI {
    static itemsTexture = texture("ui/items.png");
    static itemTile = 16;

    drawItem(id, count, x, y) {
        drawTile(BaseUI.itemsTexture, Number(id), BaseUI.itemTile, x, y)
        if (count) {
            ctx.font = "5px editundo";
            ctx.fillStyle = "white";
            ctx.fillText(count, x + BaseUI.itemTile + 3 - ctx.measureText(count).width, y + BaseUI.itemTile);
        }
    }

    draw() {}
    click(_event) {}
}

class InventoryUI extends BaseUI {
    static background = texture("ui/inventory.png");

    draw(width, height) {
        const dx = (width - InventoryUI.background.width) / 2;
        const dy = (height - InventoryUI.background.height) / 2;
        ctx.drawImage(InventoryUI.background, dx, dy);
        let index = 0;
        for (const [item, count] of Object.entries(player.inventory)) {
            this.drawItem(item, count == 1 ? null : count, dx + 20 + index * (1 + BaseUI.itemTile), dy + 19);
            index++;
        }
    }
}

/**
 * @type {BaseUI | null}
 */
let currentUI = null;
