<script>
    import * as SingleContainer from "$lib/components/layout/single";
    import * as DoubleContainer from "$lib/components/layout/double";
    import BaseChart from "$lib/components/charts/line/BaseChart.svelte";
    import { page } from '$app/stores';
    import { onMount } from "svelte";
    import { invalidate } from "$app/navigation";
    import CandleChart from "$lib/components/charts/line/CandleChart.svelte";
    import {CandleBundle} from "$lib/core/candle.js";
    import AreaChart from "$lib/components/charts/line/AreaChart.svelte";
    import BarChart from "$lib/components/charts/line/BarChart.svelte";

    let categoryRealtimeValueHistory = null; 
    let categoryDailyValueHistory = null; 
    let categoryAreaValueHistory = null;

    onMount(async () => {
        const interval = setInterval(() => {
            invalidate('data:categoryValue');
        }, 1000 * 60);

        categoryRealtimeValueHistory = new CandleBundle()
            .add(await $page.data.realtimeCategoryValueHistory)
            .evaluate();

        categoryAreaValueHistory = new CandleBundle()
            .add(await $page.data.categoryValueHistory)
            .evaluate("day", true);

        categoryDailyValueHistory = new CandleBundle()
            .add(await $page.data.categoryValueHistory)
            .evaluate("day");

        return () => {
            clearInterval(interval);
        }
    });
</script>

<SingleContainer.Root>
    <SingleContainer.Title>분석</SingleContainer.Title>
    <div/>
</SingleContainer.Root>

<DoubleContainer.Root>
    <DoubleContainer.ContentIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>실시간</h2>
            {#if categoryRealtimeValueHistory === null}
            <CandleChart />
            {:else}
            <CandleChart data={categoryRealtimeValueHistory.history} />
            {/if}
        </DoubleContainer.InnerIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>일간</h2>
            {#if categoryDailyValueHistory === null}
            <BaseChart />
            {:else}
            <BaseChart data={categoryDailyValueHistory.history} timeScale="date" />
            {/if}
        </DoubleContainer.InnerIndependent>
    </DoubleContainer.ContentIndependent>
    <DoubleContainer.ContentIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>카테고리별</h2>
            {#if categoryAreaValueHistory === null}
            <AreaChart timeScale="date" />
            {:else}
            <AreaChart data={categoryAreaValueHistory} timeScale="date" />
            {/if}
        </DoubleContainer.InnerIndependent>
        <DoubleContainer.InnerIndependent>
            <h2>누적 수익</h2>
            {#if categoryAreaValueHistory === null }
            <BarChart timeScale="date" />
            {:else}
            <BarChart data={categoryAreaValueHistory} timeScale="date" />
            {/if}
        </DoubleContainer.InnerIndependent>
    </DoubleContainer.ContentIndependent>
</DoubleContainer.Root>