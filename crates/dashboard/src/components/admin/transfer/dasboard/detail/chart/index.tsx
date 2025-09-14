import ChartMonthTransferAmountSender from "./ChartMonthAmountSender";
import ChartYearTransferAmountSender from "./ChartYearAmountSender";
import ChartMonthTransferAmountReceiver from "./ChartMonthAmountReceiver";
import ChartYearTransferAmountReceiver from "./ChartYearAmountReceiver";

interface ChartDashboardTransferDetailProps {
    loadingMonthTransferAmountSender: boolean;
    loadingYearTransferAmountSender: boolean;
    monthlyAmountSender: any;
    yearlyAmountSender: any;
    loadingMonthTransferAmountReceiver: boolean;
    loadingYearTransferAmountReceiver: boolean;
    monthlyAmountReceiver: any;
    yearlyAmountReceiver: any;
}

export default function ChartDashboarTransferDetail({
    loadingMonthTransferAmountSender,
    loadingYearTransferAmountSender,
    monthlyAmountSender,
    yearlyAmountSender,
    loadingMonthTransferAmountReceiver,
    loadingYearTransferAmountReceiver,
    monthlyAmountReceiver,
    yearlyAmountReceiver,
}: ChartDashboardTransferDetailProps) {

    return (
        <>
            <ChartMonthTransferAmountSender loadingMonthTransferAmountSender={loadingMonthTransferAmountSender} monthlyAmount={monthlyAmountSender} />
            <ChartYearTransferAmountSender loadingYearTransferAmountSender={loadingYearTransferAmountSender} yearlyAmount={yearlyAmountSender} />
            <ChartMonthTransferAmountReceiver loadingMonthTransferAmountReceiver={loadingMonthTransferAmountReceiver} monthlyAmount={monthlyAmountReceiver} />
            <ChartYearTransferAmountReceiver loadingYearTransferAmountReceiver={loadingYearTransferAmountReceiver} yearlyAmount={yearlyAmountReceiver} />
        </>
    )
}