export interface Transactions {
    data: {
        page: number,
        perPage: number,
        totalItems: number,
        totalPages: number,
        items: Items[]
    }
}

export interface Items {
    account_id: string,
    amount: number,
    category: string[],
    category_id: number,
    collection_id: string,
    collectionName: string,
    created: Date,
    date: Date,
    has_more: boolean,
    id: string,
    merchant_name: string,
    next_cursor: string,
    pending: boolean,
    updated: Date,
    expand: {}
}
  