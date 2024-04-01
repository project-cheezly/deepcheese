<script>
    import * as Dialog from "$lib/components/create-update-catalog";
    export let closeDialog;
    export let assets, accounts, categories;

    $: assetList = Object.entries(assets).reduce((acc, [key, value]) => {
        acc.push({id: key, name: value});
        return acc;
    }, []);

    $: accountList = Object.entries(accounts).reduce((acc, [key, value]) => {
        acc.push({id: key, name: value});
        return acc;
    }, []);

    $: categoryList = Object.entries(categories).reduce((acc, [key, value]) => {
        acc.push({id: key, name: value});
        return acc;
    }, []);

    const tradeType = [
        { id: '1', name: '매수' },
        { id: '2', name: '매도' },
        { id: '3', name: '배당/이자' },
        { id: '4', name: '무상증자' }
    ];
</script>

<h3 class="py-0">추가</h3>
<form method="POST" action='?/create' class="pt-4">
    <div class="space-y-4">
        <Dialog.DateInput />
        <Dialog.SelectInput itemList={assetList} name="asset_id" />
        <Dialog.TouchSelectInput items={tradeType} name="trade_type" title="거래 분류" />
        <Dialog.TouchSelectInput items={accountList} name="account_id" title="계좌" />
        <Dialog.TouchSelectInput items={categoryList} name="category_id" title="카테고리" />
        <Dialog.TextInput name="amount" placeholder="수량"/>
        <Dialog.TextInput name="value" placeholder="가격"/>
        <Dialog.TextInput name="fee" placeholder="수수료"/>
        <div class="text-right space-x-4 py-2">
            <button class="button-small text-gray-400" on:click|preventDefault={closeDialog}>취소</button>
            <button class="button-small">추가</button>
        </div>
    </div>
</form>
