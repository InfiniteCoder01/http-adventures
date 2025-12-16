class BaseUI {
    static itemsTexture = texture("ui/items.png");
    static itemTile = 16;
    static background = texture("ui/inventory.png");

    drawItem(id, count, x, y) {
        drawTile(BaseUI.itemsTexture, Number(id), BaseUI.itemTile, x, y)
        if (count) {
            ctx.font = "5px editundo";
            ctx.fillStyle = "white";
            ctx.fillText(count, x + BaseUI.itemTile - ctx.measureText(count).width, y + BaseUI.itemTile);
        }
    }

    drawBaseInventory(x, y) {
        ctx.drawImage(InventoryUI.background, x, y);
        let index = 0;
        for (const [item, count] of Object.entries(player.inventory)) {
            const col = index % 5;
            const row = Math.floor(index / 5);
            this.drawItem(
                item,
                count == 1 ? null : count,
                x + 16 + col * 20,
                y + 16 + row * 20
            );
            index++;
        }
    }

    draw() {}
    click(_event) {}
}

class InventoryUI extends BaseUI {
    draw(width, height) {
        const dx = (width - BaseUI.background.width) / 2;
        const dy = (height - BaseUI.background.height) / 2;
        this.drawBaseInventory(dx, dy);
    }
}

/**
 * @type {BaseUI | null}
 */
let currentUI = null;
