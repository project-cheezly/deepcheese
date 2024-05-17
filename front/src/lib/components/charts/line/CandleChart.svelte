<script>
    import * as d3 from 'd3';
    import { multiFormat, localeMultiFormat } from '../time_format_locale.js';

    export let data = [];

    export let prefixYFormat = (value) => {
        if (value < 1000) {
            return new Intl.NumberFormat(value);
        } else if (value < 100000000) {
            return `${new Intl.NumberFormat().format(Math.round(value / 10000))}만`;
        } else {
            return `${new Intl.NumberFormat().format(value / 100000000)}억`;
        }
    }

    export let timeScale = 'time';

    let width = 720;
    let height = 480;
    let margin = { top: 30, right: 80, bottom: 30, left: 5 };

    let buckets = [];

    $: {
        const bin = d3.bin()
            .value(d => d[0])
            .thresholds((data, min, max) =>
                d3.scaleUtc()
                    .domain([min, max])
                    .ticks(d3.utcHour.every(1))
            );

        buckets = bin(data)
            .map((d) => [d3.extent(d, (d) => d[1]), d3.min(d, (d) => d[0])])
            .filter((d) => d[0][0] !== undefined);
    }

    // x, y 차트 범위 설정
    let xRange, yRange;
    $: {
        xRange = d3.extent(data, (d) => d[0]);
        yRange = d3.extent(data, (d) => d[1]);

        xRange[0] = xRange[0].set('minute', 0).set('second', 0).add(0, 'minute');
        xRange[1] = xRange[1].set('minute', 0).set('second', 0).add(40, 'minute');

        const yRangeDiff = yRange[1] - yRange[0];
        yRange[0] = Math.max((yRange[0] - yRangeDiff * 0.1), 0);
        yRange[1] += yRangeDiff * 0.1;
    }

    // x, y 축 생성
    let xAxisLine, yAxisLine;

    $: x = d3.scaleTime(xRange, [margin.left, width - margin.right]);
    $: y = d3.scaleLinear(yRange, [height - margin.bottom, margin.top]);

    $: {
        d3.select(xAxisLine)
            .call(d3.axisBottom(x)
                .ticks(timeScale === 'time' ? 4 : d3.utcMonth)
                .tickFormat(timeScale === 'time' ? localeMultiFormat : multiFormat)
                .tickSize(0)
                .tickPadding(8)
                .tickSizeOuter(0));

        d3.select(xAxisLine)
            .selectAll('.tick text')
            .attr('text-anchor', 'start')
            .attr('dx', '0.5em');

        d3.select(yAxisLine)
            .call(d3.axisRight(y)
                .ticks(4)
                .tickSize(0)
                .tickFormat(prefixYFormat)
                .tickPadding(8)
                .tickSizeOuter(0)
                .tickPadding(6));
    }

    $: xTicks = x.ticks(timeScale === 'time' ? 4 : d3.utcMonth);
    $: yTicks = y.ticks(4);

</script>

<svg viewBox="0 0 {width} {height}">
    <g
        bind:this={xAxisLine}
        transform={`translate(0, ${height - margin.bottom})`}
        class="text-gray-400 text-xl md:text-lg lg:text-base"
    />
    <g
        bind:this={yAxisLine}
        transform={`translate(${width - margin.right}, 0)`}
        class="text-gray-400 text-xl md:text-lg lg:text-base"
    />
    {#each xTicks as tick}
        <line
            transform={`translate(${x(tick)}, ${margin.top})`}
            x2="0"
            y2="{height - margin.bottom - margin.top + 20}"
            stroke="var(--md-sys-color-outline)"
            stroke-dasharray="4"
        />
    {/each}
    {#each yTicks as tick}
        <line
            transform={`translate(${margin.left}, ${y(tick)})`}
            x2="{width - margin.right - margin.left}"
            y2="0"
            stroke="var(--md-sys-color-outline)"
            stroke-width="0.5"
        />
    {/each}
    <g
        fill="var(--md-sys-color-inverse-primary)"
        stroke="currentColor"
        stroke-width="0"
    >
        {#each buckets as [[maxVal, minVal], timestamp]}
            <rect
                x={x(timestamp.set('minute', 0).set('second', 0)) - 6}
                y={maxVal === minVal ? y(minVal) - 6 : y(minVal) - 3}
                width="12"
                height="{Math.max(y(maxVal) - y(minVal) + 3, 12)}"
                rx="6"
                ry="6"
            />
        {/each}
        <circle
            cx={x(data[data.length - 1][0].set('minute', 0))}
            cy={y(data[data.length - 1][1])}
            r="6"
            fill="var(--md-sys-color-primary)"
        />
        <g>
            <rect
                x={width - margin.right}
                y={y(data[data.length - 1][1]) - height * 0.04}
                width={margin.right + 5}
                height='2.25em'
                fill="var(--md-sys-color-primary-container)"
            />
            <text
                x={width - margin.right + 6}
                y={y(data[data.length - 1][1]) + height * 0.01}
                text-anchor="start"
                fill="var(--md-sys-color-on-primary-container)"
                class="text-xl lg:text-base"
            >{prefixYFormat(data[data.length - 1][1])}</text>
        </g>
    </g>
</svg>