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

    // x, y 차트 범위 설정
    let xRange, yRange;
    $: {
        xRange = d3.extent(data, (d) => d[0]);
        yRange = d3.extent(data, (d) => d[1]);

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

    // 꺾은선 생성
    $: line = d3.line()
        .x((d) => x(d[0]))
        .y((d) => y(d[1]));
</script>

<svg viewBox="0 0 {width} {height}">
    <path
        d={line(data)}
        fill="transparent"
        stroke="var(--color-primary)"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
    />
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
            stroke="rgb(156, 163, 175)"
            stroke-dasharray="4"
        />
    {/each}
    {#each yTicks as tick}
        <line
            transform={`translate(${margin.left}, ${y(tick)})`}
            x2="{width - margin.right - margin.left}"
            y2="0"
            stroke="rgb(156, 163, 175)"
            stroke-width="0.5"
        />
    {/each}
    <g
        fill="white"
        stroke="currentColor"
        stroke-width="3"
    >
        {#if data.length < 51}
            {#each data as d}
                    <circle
                        cx={x(d[0])}
                        cy={y(d[1])}
                        r="4"
                    />
            {/each}
        {/if}
    </g>
    <g>
        {#if data.length > 0}
        <rect
            x={width - margin.right}
            y={y(data[data.length - 1][1]) - height * 0.04}
            width={margin.right + 5}
            height='2.25em'
            fill="var(--color-outline)"
        />
        <text
            x={width - margin.right + 6}
            y={y(data[data.length - 1][1]) + height * 0.01}
            text-anchor="start"
            fill="var(--color-background)"
            class="text-xl lg:text-base"
        >{prefixYFormat(data[data.length - 1][1])}</text>
        {/if}
    </g>
</svg>