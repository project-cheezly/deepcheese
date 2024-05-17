<script>
    import * as d3 from 'd3';
    import { multiFormat, localeMultiFormat } from '../time_format_locale.js';

    const colors = ["#4e79a7","#f28e2c","#e15759","#76b7b2","#59a14f","#edc949","#af7aa1","#ff9da7","#9c755f","#bab0ab"];

    export let data = [];

    let initialData = new Map();

    $: {
        for (const row in data) {
            if (!initialData.has(data[row][2])) {
                initialData.set(data[row][2], data[row][1]);
            }
        }
    }

    $: series = d3.stack()
        .keys(d3.union(data.map(d => d[2])))
        .value(([, group], key) => group.get(key)[1] - initialData.get(key))
        .offset(d3.stackOffsetDiverging)
        (d3.index(data, d => d[0], d => d[2]));

    export let prefixYFormat = (value) => {
        if (Math.abs(value) < 1000) {
            return new Intl.NumberFormat().format(value)
        } else if (Math.abs(value) < 100000000) {
            return `${new Intl.NumberFormat().format(Math.round(value / 10000))}만`;
        } else {
            return `${new Intl.NumberFormat().format(value / 100000000)}억`;
        }
    }

    export let timeScale = 'time';


    let width = 720;
    let height = 480;
    let margin = { top: 30, right: 80, bottom: 30, left: 5 };

    // x, y 차트 범위 설정
    let xRange, yRange;
    $: {
        xRange = d3.extent(data, (d) => d[0]);
        yRange = d3.extent(data, (d) => d[1]);

        yRange[0] = d3.min(series, (d) => d3.min(d, (d) => d[0]));
        yRange[1] = d3.max(series, (d) => d3.max(d, (d) => d[1]));

        const yDiff = yRange[1] - yRange[0];
        yRange[0] -= yDiff * 0.1;
        yRange[1] += yDiff * 0.1;
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
            .attr('dx', '0.5rem');

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
    {#each series as col, idx}
        {#each col as val}
            <rect
                x="{x(val.data[0]) - width / col.length * 0.7}"
                y="{y(val[1])}"
                width={width / col.length * 0.7}
                height="{Math.abs(y(val[0]) - y(val[1]))}"
                fill={colors[idx]}
                fill-opacity="0.4a"
            />
        {/each}
    {/each}
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
</svg>