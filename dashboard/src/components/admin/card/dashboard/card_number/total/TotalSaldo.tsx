import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Wallet } from "lucide-react";

export default function TotalSaldoCard({ total_balance }: { total_balance: string }) {
    return (
        <Card className="w-full shadow-lg rounded-md border p-4">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                    Total Saldo
                </CardTitle>
                <Wallet className="h-6 w-6 text-gray-500" />
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">
                    {total_balance}
                </div>
            </CardContent>
        </Card>
    )
}