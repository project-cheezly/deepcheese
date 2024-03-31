<script>
    import * as Dialog from '$lib/components/create-update-catalog';
    export let name, number, closeDialog;
</script>

<h3 class="py-0">계좌 추가</h3>
<form method="POST" action="?/create" class="pt-4">
    <div class="space-y-4">
        <Dialog.TextInput name='name' placeholder="이름" value={name} />
        <Dialog.TextInput name='number' placeholder="계좌번호" value={number} />
        <div class="text-right space-x-4 py-2">
            <button class="button-small text-gray-400" on:click|preventDefault={closeDialog}>취소</button>
            <button class="button-small">추가</button>
        </div>
    </div>
</form>