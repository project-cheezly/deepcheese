<script>
    import BaseChart from "$lib/components/charts/line/BaseChart.svelte";
    import * as Container from '$lib/components/double-layer-container';
    import { page } from '$app/stores';
    import { onMount } from "svelte";
    import { invalidate } from "$app/navigation";
    import CandleChart from "$lib/components/charts/line/CandleChart.svelte";
    import {CandleBundle} from "$lib/core/candle.js";
    import AreaChart from "$lib/components/charts/line/AreaChart.svelte";
    import BarChart from "$lib/components/charts/line/BarChart.svelte";

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

<div class="container">
    <h1 class="main-content-large">분석</h1>
    <div/>
</div>

<Container.Root>
    <Container.Content>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>실시간</h2>
            <CandleChart data={categoryRealtimeValueHistory.history} />
        </div>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>일간</h2>
            <BaseChart data={categoryDailyValueHistory.history} timeScale="date" />
        </div>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>카테고리별</h2>
            <AreaChart data={categoryAreaValueHistory} timeScale="date" />
        </div>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>누적 수익</h2>
            <BarChart data={categoryAreaValueHistory} timeScale="date" />
        </div>
    </Container.Content>
</Container.Root>
