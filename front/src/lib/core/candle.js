import moment from "moment";

export class Candle {
    /*
    * @constructor
     */
    constructor() {
        /* @type {Array<Moment, float>} */
        this.history = [];
    }

    /*
    * @param {Moment} timestamp
    * @param {float} value
     */
    add(timestamp, value) {
        this.history.push([timestamp, value]);
    }
}

export class CandleBundle {
    /*
    * @constructor
    *
    * @param {Object[]} data
     */
    constructor() {
        /* @type {Map<string, Candle>} */
        this.candles = new Map();
    }

    /*
    * @param {Object[]} data
     */
    add(data) {
        data.sort((a, b) => {
            return moment(a.timestamp) - moment(b.timestamp);
        });

        for (let i = 0; i < data.length; i++) {
            let timestamp = moment(data[i].timestamp);
            let category = data[i].category_name;
            let value = parseFloat(data[i].value);

            if (!this.candles.has(category)) {
                this.candles.set(category, new Candle());
            }

            this.candles.get(category).add(timestamp, value);
        }

        return this;
    }

    /*
    * @param {function} key_func
     */
    evaluate(interval="minute", stacked=false) {
        let categoryProperties = new Map();
        this.candles.forEach((candle, category) => {
            let history = this.candles.get(category).history;

            categoryProperties.set(category, {
                prevValue: history[0][1],
                idx: 0,
                closed: history[history.length - 1][0],
                opened: history[0][0],
            })
        });

        let prevDate = moment.max();
        categoryProperties.forEach((properties, _) => {
            prevDate = moment.min(prevDate, properties.opened);
        });

        let _key_func = (x) => x[0];

        let res = !stacked ? new Candle() : [];

        while (true) {
            let oldest_category = "";
            let oldest_key = _key_func([moment("9999-12-31"), 0]);

            for (const category of categoryProperties.keys()) {
                let properties = categoryProperties.get(category);
                const history = this.candles.get(category).history;

                while (
                    properties.idx < history.length - 1 &&
                    _key_func(history[properties.idx]).isSame( _key_func(history[properties.idx + 1]), interval)
                ) {
                    properties.prevValue = history[properties.idx][1];
                    properties.idx += 1;
                }
            }

            categoryProperties.forEach((properties, category) => {
                if (properties.idx >= this.candles.get(category).history.length) {
                    return;
                }

                let key = _key_func(this.candles.get(category).history[properties.idx]);
                if (key < oldest_key) {
                    oldest_category = category;
                    oldest_key = key;
                }
            });

            let value = !stacked ? 0 : [];

            if (!stacked) {
                for (const category of categoryProperties.keys()) {
                    const properties = categoryProperties.get(category);
                    const history = this.candles.get(category).history;

                    if (
                        properties.idx < history.length &&
                        _key_func(history[properties.idx]).isSame(oldest_key, interval)
                    ) {
                        value += history[properties.idx][1];
                        properties.prevValue = history[properties.idx][1];
                        properties.idx += 1;
                    } else {
                        value += properties.prevValue;
                    }
                }
            } else {
                for (const category of categoryProperties.keys()) {
                    const properties = categoryProperties.get(category);
                    const history = this.candles.get(category).history;

                    if (
                        properties.idx < history.length &&
                        _key_func(history[properties.idx]).isSame(oldest_key, interval)
                    ) {
                        value.push([oldest_key, history[properties.idx][1], category]);
                        properties.prevValue = history[properties.idx][1];
                        properties.idx += 1;
                    } else {
                        value.push([oldest_key, properties.prevValue, category]);
                    }
                }
            }

            if (!stacked) {
                res.add(oldest_key, value);
            } else {
                for (const v of value) {
                    res.push(v);
                }
            }

            let finished = true;
            for (const category of categoryProperties.keys()) {
                if (categoryProperties.get(category).idx < this.candles.get(category).history.length) {
                    finished = false;
                    break;
                }
            }

            if (finished) {
                break;
            }
        }

        return res;
    }
}