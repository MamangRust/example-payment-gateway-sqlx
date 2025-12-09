import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ArrowDownCircle } from "lucide-react";

export default function TotalWithdrawCard({ total_withdraw }: { total_withdraw: string }) {
    return (
        <Card className="w-full shadow-lg rounded-md border p-4">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                    Total Withdraw
                </CardTitle>
                <ArrowDownCircle className="h-6 w-6 text-blue-500" />
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">
                    {total_withdraw}
                </div>
            </CardContent>
        </Card>

    )
}