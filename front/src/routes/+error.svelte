<script>
    import { page } from '$app/stores';
</script>

<div class="container main-container">
    <div class="main-content-large">
        <h1>{$page.status}</h1>
        <p class="text-lead ml-1.5">{$page.error.message}</p>
    </div>
    <div />
</div>
