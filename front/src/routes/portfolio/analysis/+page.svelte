<script>
    import * as SingleContainer from "$lib/components/layout/single";
    import * as DoubleContainer from "$lib/components/layout/double";
    import BaseChart from "$lib/components/charts/line/BaseChart.svelte";
    import * as Container from '$lib/components/double-layer-container';
    import { page } from '$app/stores';
    import { onMount } from "svelte";
    import { invalidate } from "$app/navigation";
    import CandleChart from "$lib/components/charts/line/CandleChart.svelte";
    import {CandleBundle} from "$lib/core/candle.js";
    import AreaChart from "$lib/components/charts/line/AreaChart.svelte";
    import BarChart from "$lib/components/charts/line/BarChart.svelte";
    import ContentIndependent from "../../../lib/components/layout/double/ContentIndependent.svelte";
    import InnerIndependent from "../../../lib/components/layout/double/InnerIndependent.svelte";

    onMount(() => {
        const interval = setInterval(() => {
            invalidate('data:categoryValue');
        }, 1000 * 60);

        return () => {
            clearInterval(interval);
        }
    });

    $: categoryRealtimeValueHistory = new CandleBundle()
        .add($page.data.realtimeCategoryValueHistory)
        .evaluate();

    $: categoryDailyValueHistory = new CandleBundle()
        .add($page.data.categoryValueHistory)
        .evaluate("day");

    $: categoryAreaValueHistory = new CandleBundle()
        .add($page.data.categoryValueHistory)
        .evaluate("day", true);
</script>

<SingleContainer.Root>
    <SingleContainer.Title>분석</SingleContainer.Title>
    <div/>
</SingleContainer.Root>

<DoubleContainer.Root>
    <DoubleContainer.ContentIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>실시간</h2>
            <CandleChart data={categoryRealtimeValueHistory.history} />
        </DoubleContainer.InnerIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>일간</h2>
            <BaseChart data={categoryDailyValueHistory.history} timeScale="date" />
        </DoubleContainer.InnerIndependent>
    </DoubleContainer.ContentIndependent>
    <DoubleContainer.ContentIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>카테고리별</h2>
            <AreaChart data={categoryAreaValueHistory} timeScale="date" />
        </DoubleContainer.InnerIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>누적 수익</h2>
            <BarChart data={categoryAreaValueHistory} timeScale="date" />
        </DoubleContainer.InnerIndependent>
    </DoubleContainer.ContentIndependent>
</DoubleContainer.Root>
