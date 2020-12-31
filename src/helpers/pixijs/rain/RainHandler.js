
import * as PIXI from "pixi.js";
import RainDrop from "./RainDrop"


PIXI.Application.prototype.rain = function () {
    let lastDrop = new Date().getTime();
    /**@const {Array<RainDrop>}*/
    let RainDrops = [];
    const graphics = new PIXI.Graphics();
    this.renderer.width
    this.stage.addChild(graphics);

    const animate = () => {
        graphics.clear();
        if (RainDrops.length < 10 && (new Date().getTime() - lastDrop) > 1000 && Math.random()>0.99) {
            lastDrop = new Date().getTime();
            RainDrops.push(new RainDrop(graphics, {
                containerWidth: this.renderer.width / this.renderer.resolution,
                onDestroy: (e) => RainDrops = RainDrops.filter(a => a !== e)
            }))
        }
        RainDrops.forEach(raindrop => raindrop.next());
        requestAnimationFrame(animate);
    }
    animate();
}