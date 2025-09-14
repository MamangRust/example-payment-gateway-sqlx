import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ChevronDown } from "lucide-react";
import { Table } from "@/components/ui/table";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Skeleton } from "@/components/ui/skeleton";
import TableHeaderCard from "./table-header";
import TableBodyCard from "./table-body";
import TableFooterCard from "./table-footer";

const TableCardTrashed = ({
  search,
  setSearch,
  isLoadingWithDelay,
  loadingGetCardsTrashed,
  table,
  pagination,
  handlePageChange,
  handlePageSizeChange,
}: any) => {
  return (
    <Card className="w-full shadow-lg rounded-md border">
      <CardHeader className="p-4">
        <div className="flex justify-between items-center">
          <h3 className="text-lg font-semibold">Table Card</h3>
        </div>
      </CardHeader>
      <CardContent className="p-4">
        <div className="flex items-center py-4">
          <Input
            placeholder="Filter by card number"
            value={search}
            onChange={(event) => setSearch(event.target.value)}
            className="max-w-sm"
          />
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" className="ml-auto">
                Columns <ChevronDown className="ml-2 h-4 w-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              {table
                .getAllColumns()
                .filter((column: any) => column.getCanHide())
                .map((column: any) => (
                  <DropdownMenuCheckboxItem
                    key={column.id}
                    className="capitalize"
                    checked={column.getIsVisible()}
                    onCheckedChange={(value: any) =>
                      column.toggleVisibility(!!value)
                    }
                  >
                    {column.id}
                  </DropdownMenuCheckboxItem>
                ))}
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        {isLoadingWithDelay || loadingGetCardsTrashed ? (
          <div className="space-y-4">
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
          </div>
        ) : (
          <div className="rounded-md border h-[525px] overflow-y-scroll">
            <Table>
              <TableHeaderCard table={table} />
              <TableBodyCard table={table} />
            </Table>
          </div>
        )}
      </CardContent>
      <CardFooter>
        <TableFooterCard
          table={table}
          pagination={{
            currentPage: pagination.currentPage,
            pageSize: pagination.pageSize,
            totalItems: pagination.totalItems,
            totalPages: pagination.totalPages,
          }}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      </CardFooter>
    </Card>
  );
};

export default TableCardTrashed;
