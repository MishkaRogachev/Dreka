
export interface LongpressConfig {
    delay: number;
    repeat: boolean;
    onLongPress: () => void;
    onIdleClick?: () => void;
    onDropped?: () => void;
}

export function longpress(node: any, config: LongpressConfig) {
    let timer: NodeJS.Timeout | undefined;
    let fired = false;

    const onMouseDown = () => {
        fired = false;
        timer = config.repeat ?
            setInterval(() => { config.onLongPress(); fired = true; }, config.delay) :
            setTimeout(() => { config.onLongPress(); fired = true; }, config.delay);
    };

    const onMouseUp = () => {
        if (!fired && config.onIdleClick) {
            config.onIdleClick();
        }
        dropDelay();
    };

    const dropDelay = () => {
        config.repeat ? clearInterval(timer) : clearTimeout(timer);
        timer = undefined;
        fired = false;
        if (config.onDropped) {
            config.onDropped();
        }
    };

    node.addEventListener('mousedown', onMouseDown);
    node.addEventListener('mouseup', onMouseUp);
    node.addEventListener('mouseleave', dropDelay);

    return {
        update(newConfig: LongpressConfig) {
            dropDelay();
            config = newConfig;
        },
        destroy() {
            dropDelay();
            node.removeEventListener('mousedown', onMouseDown);
            node.removeEventListener('mouseup', onMouseUp);
            node.removeEventListener('mouseleave', dropDelay);
        }
    };
}
