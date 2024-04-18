
export interface LongpressConfig {
    delay: number;
    repeat: boolean;
    onLongPress: () => void;
}

export function longpress(node: any, config: LongpressConfig) {
    let timer: NodeJS.Timeout;

    const startTimer = () => {
        timer = config.repeat ?
            setInterval(() => { config.onLongPress(); }, config.delay) :
            setTimeout(() => { config.onLongPress(); }, config.delay);
    };

    const clearTimer = () => {
        config.repeat ? clearInterval(timer) : clearTimeout(timer);
    };

    node.addEventListener('mousedown', startTimer);
    node.addEventListener('mouseup', clearTimer);
    node.addEventListener('mouseleave', clearTimer);

    return {
        update(newConfig: LongpressConfig) {
            clearTimer();
            config = newConfig;
        },
        destroy() {
            clearTimer();
            node.removeEventListener('mousedown', startTimer);
            node.removeEventListener('mouseup', clearTimer);
            node.removeEventListener('mouseleave', clearTimer);
        }
    };
}