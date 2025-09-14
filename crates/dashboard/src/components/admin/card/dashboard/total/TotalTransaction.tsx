import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { CreditCard } from "lucide-react";

export default function TotalTransactionCard({ total_transaction }: { total_transaction: string }) {
    return (
        <Card className="w-full shadow-lg rounded-md border p-4">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                    Total Transaction
                </CardTitle>
                <CreditCard className="h-6 w-6 text-purple-500" />
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">
                    {total_transaction}
                </div>
            </CardContent>
        </Card>
    )
}