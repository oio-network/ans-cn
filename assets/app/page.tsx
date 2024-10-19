import { asnSchema } from '@/schema/schema'
import { z } from 'zod'

import { columns } from '@/components/data/columns'
import { DataTable } from '@/components/data/data-table'

async function getASNs() {
  const data = await fetch('https://asn-cn.oio.sd/api/asns', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  })
    .then((res) => res.json())
    .catch((err) => {
      console.error(err)
    })

  return z.array(asnSchema).parseAsync(data)
}

export default async function Page() {
  const asns = await getASNs()

  return (
    <div className="container relative">
      <div className="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
        <div className="flex items-center justify-between space-y-2">
          <div>
            <h2 className="text-2xl font-bold tracking-tight">Welcome back!</h2>
            <p className="text-muted-foreground">
              Here&apos;s a list of China mainland ASNs classified by major China ISPs!
            </p>
          </div>
        </div>
        <DataTable data={asns} columns={columns} />
      </div>
    </div>
  )
}
