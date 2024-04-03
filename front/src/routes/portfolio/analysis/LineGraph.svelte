<script>
    function setPrefix(value) {
        if (value < 1000) {
            return value;
        } else if (value < 100000000) {
            return `${value / 10000}만`;
        } else {
            return `${value / 100000000}억`;
        }
    }

    import * as d3 from 'd3';

    export let data = [];

    export let width = 720;

    export let height = 480;

    export let margin = [30, 100, 30, 5]; // top, right, bottom, left
    let gx, gy;

    $: yRange = d3.extent(data, (d) => parseFloat(d.value));
    $: {
        const yRangeDiff = yRange[1] - yRange[0];
        yRange[0] -= yRangeDiff * 0.5;
        yRange[1] += yRangeDiff * 0.5;
    }
    // $: xRange = [new Date("2024-04-02"), new Date("2024-04-03")]
    $: xRange = d3.extent(data, (d) => d.tr_timestamp);

    $: x = d3.scaleUtc(xRange, [margin[3], width - margin[1]]);
    $: y = d3.scaleLinear(yRange, [height - margin[2], margin[0]]);

    $: newLine = d3.line()
        .x((d) => x(d.tr_timestamp))
        .y((d) => y(d.value))
        .curve(d3.curveCatmullRom.alpha(0.4));

    $: d3.select(gx)
        .call(d3.axisBottom(x)
            .ticks(d3.timeHour)
            .tickSize(0)
            .tickFormat((d) => { console.log(d); return d3.timeFormat('%H시')(d); })
            // .tickFormat(d3.utcFormat('%m월 %d일'))
            .tickPadding(8)
            .tickSizeOuter(0));

    $: xTicks = x.ticks(d3.timeHour);
    $: yTicks = y.ticks(4);

    $: d3.select(gx)
        .selectAll('.tick text')
        .attr('dx', '1.5em');

    $: ticks = d3.select(gy)
        .call(d3.axisRight(y)
            .ticks(4)
            .tickFormat(setPrefix)
            .tickSizeOuter(0)
            .tickSize(0)
            .tickPadding(6));
</script>

<svg viewBox="0 0 {width} {height}">
    <path
        d={newLine(data)}
        fill="transparent"
        stroke="rgb(35, 35, 35)"
        stroke-width="3"
        stroke-linecap="round"
        stroke-linejoin="round"
    />
    <g
        bind:this={gx}
        transform={`translate(0, ${height - margin[2]})`}
        class="text-gray-400 text-xl md:text-lg lg:text-base"
    />
    <g
        bind:this={gy}
        transform={`translate(${width - margin[1]}, 0)`}
        class="text-gray-400 text-xl md:text-lg lg:text-base"
    >
    </g>
    {#each xTicks as tick, i}
        <line
            transform={`translate(${x(tick)}, ${margin[0]})`}
            x2="0"
            y2="{height - margin[2] - margin[0] + 20}"
            stroke="rgb(156, 163, 175)"
            stroke-dasharray="4"
        />
    {/each}
    {#each yTicks as tick, i}
        <line
            transform={`translate(${margin[3]}, ${y(tick)})`}
            x2="{width - margin[1] - margin[3]}"
            y2="0"
            stroke="rgb(156, 163, 175)"
            stroke-width="0.5"
        />
    {/each}
</svg>