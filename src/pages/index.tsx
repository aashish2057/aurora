import Head from "next/head"
import Link from "next/link"

import { siteConfig } from "@/config/site"
import { Layout } from "@/components/layout"
import { buttonVariants } from "@/components/ui/button"
import { pb } from "./api/pocketbase/pocketbase"

export async function getServerSideProps() {
  const response = await fetch("http://127.0.0.1:8090/api/collections/transactions/records")
  const data = await response.json()
  return {props: {data}}
}

export default function IndexPage({ data }) {
  console.log(data)
  return (
    <Layout>
      <Head>
        <title>Aurora</title>
        <meta name="description" content="Financial Dashbaord" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
    </Layout>
  )
}
