import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ArrowUpCircle } from "lucide-react";


export default function TotalTopupCard({ total_topup }: { total_topup: string }) {
    return (
        <Card className="w-full shadow-lg rounded-md border p-4">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                    Total Topup
                </CardTitle>
                <ArrowUpCircle className="h-6 w-6 text-orange-500" />
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">
                    {total_topup}
                </div>
            </CardContent>
        </Card>
    )
}