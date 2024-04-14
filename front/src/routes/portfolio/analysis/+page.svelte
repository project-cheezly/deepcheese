<script>
    import BaseChart from "$lib/components/charts/line/BaseChart.svelte";
    import * as Container from '$lib/components/double-layer-container';
    import { page } from '$app/stores';
    import { onMount } from "svelte";
    import { invalidate } from "$app/navigation";
    import CandleChart from "$lib/components/charts/line/CandleChart.svelte";

    onMount(() => {
        const interval = setInterval(() => {
            invalidate('data:categoryValue');
        }, 1000 * 60);

        return () => {
            clearInterval(interval);
        }
    })
</script>

<div class="container">
    <h1 class="main-content-large">분석</h1>
    <div/>
</div>

<Container.Root>
    <Container.Content>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>실시간</h2>
            <CandleChart data={$page.data.realtimeCategoryValueHistory} />
        </div>
        <div class="main-content col-span-2 md:col-span-1">
            <h2>일간</h2>
            <BaseChart data={$page.data.categoryValueHistory} timeScale="date" />
        </div>
    </Container.Content>
</Container.Root>
