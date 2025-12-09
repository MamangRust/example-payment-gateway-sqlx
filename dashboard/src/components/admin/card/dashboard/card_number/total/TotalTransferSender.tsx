import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Send } from "lucide-react";

export default function TotalTransferSender({ total_sender }: { total_sender: string }) {
    return (
        <Card className="w-full shadow-lg rounded-md border p-4">
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                    Total Transfer Sender
                </CardTitle>
                <Send className="h-6 w-6 text-green-500" />
            </CardHeader>
            <CardContent>
                <div className="text-2xl font-bold">
                    {total_sender}
                </div>
            </CardContent>
        </Card>
    )
}