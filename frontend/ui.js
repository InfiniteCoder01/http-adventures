class BaseUI {
    static itemsTexture = texture("ui/items.png");
    static itemTile = 16;

    drawItem(id, x, y) {
        const items = ["wood"];
        const index = items.indexOf(id);
        drawTile(BaseUI.itemsTexture, index, BaseUI.itemTile, x, y)
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
        this.drawItem("wood", dx + 20, dy + 19);
    }
}

/**
 * @type {BaseUI | null}
 */
let currentUI = null;
