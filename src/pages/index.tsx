import Head from "next/head"

import { Layout } from "@/components/layout"
import { Transactions } from "@/types/transactions"
import TransactionsScroll from "@/components/transactions"

export async function getServerSideProps() {
  const response = await fetch("http://127.0.0.1:8090/api/collections/transactions/records?perPage=100")
  const data = await response.json()
  return {props: {data}}
}

export default function IndexPage({ data }: Transactions) {
  console.log(data)
  return (
    <Layout>
      <Head>
        <title>Aurora</title>
        <meta name="description" content="Financial Dashbaord" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <TransactionsScroll data={data} />
    </Layout>
  )
}
