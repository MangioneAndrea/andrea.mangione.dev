
import * as PIXI from "pixi.js";
import { removeFromArray } from "../../CommonFunctions";
import RainDrop from "./RainDrop"


PIXI.Application.prototype.rain = function () {
    let lastDrop = new Date().getTime();
    /**@const {Array<RainDrop>}*/
    let RainDrops = [];
    const graphics = new PIXI.Graphics();
    this.stage.addChild(graphics);

    const animate = () => {
        graphics.clear();
        if (RainDrops.length < 10 && (new Date().getTime() - lastDrop) > 1500) {
            lastDrop = new Date().getTime();
            RainDrops.push(new RainDrop(graphics, (e) => RainDrops = RainDrops.filter(a => a !== e)))
        }
        RainDrops.forEach(raindrop => raindrop.next());
        requestAnimationFrame(animate);
    }
    animate();
}