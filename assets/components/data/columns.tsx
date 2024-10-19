'use client'

import { isps } from '@/schema/data'
import { ASN } from '@/schema/schema'
import { ColumnDef } from '@tanstack/react-table'

import { DataTableColumnHeader } from './data-table-column-header'

export const columns: ColumnDef<ASN>[] = [
  {
    accessorKey: 'number',
    header: ({ column }) => <DataTableColumnHeader column={column} title="ASN" />,
    cell: ({ row }) => <div className="w-[100px]">AS{row.getValue('number')}</div>,
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'name',
    header: ({ column }) => <DataTableColumnHeader column={column} title="Name" />,
    cell: ({ row }) => (
      <div className="max-w-[500px] truncate font-medium">{row.getValue('name')}</div>
    ),
  },
  {
    accessorKey: 'isp',
    header: ({ column }) => <DataTableColumnHeader column={column} title="ISP" />,
    cell: ({ row }) => {
      const isp = isps.find((isp) => isp.value === row.getValue('isp'))

      if (!isp) {
        return null
      }

      return <div className="flex w-[260px] items-center">{isp.label}</div>
    },
    filterFn: (row, id, value) => {
      return value.includes(row.getValue(id))
    },
  },
]
