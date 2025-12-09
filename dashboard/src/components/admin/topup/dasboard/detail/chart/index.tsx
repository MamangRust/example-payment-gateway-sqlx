import ChartMonthTopupAmount from "./ChartMonthAmount";
import ChartMonthTopupPaymentMethod from "./ChartMonthPaymentMethod";
import ChartYearTopupAmount from "./ChartYearAmount";
import ChartYearTopupPaymentMethod from "./ChartYearPaymentMethod";

interface ChartDashboardTopupProps {
    loadingMonthTopupAmount: boolean;
    loadingMonthTopupMethod: boolean;
    loadingYearTopupAmount: boolean;
    loadingYearTopupMethod: boolean

    monthlyAmount: any;
    monthlyPaymentMethod: any;
    yearlyAmount: any;
    yearlyPaymentMethod: any;
}

export default function ChartDashboarTopupDetail({
    loadingMonthTopupAmount,
    loadingMonthTopupMethod,
    loadingYearTopupAmount,
    loadingYearTopupMethod,
    monthlyAmount,
    monthlyPaymentMethod,
    yearlyAmount,
    yearlyPaymentMethod
}: ChartDashboardTopupProps) {

    return (
        <>
            <ChartMonthTopupPaymentMethod loadingMonthTopupMethod={loadingMonthTopupMethod} monthlyPaymentMethod={monthlyPaymentMethod} />
            <ChartYearTopupPaymentMethod loadingYearTopupMethod={loadingYearTopupAmount} yearlyPaymentMethod={yearlyPaymentMethod} />
            <ChartMonthTopupAmount loadingMonthTopupAmount={loadingMonthTopupAmount} monthlyAmount={monthlyAmount} />
            <ChartYearTopupAmount loadingYearTopupAmount={loadingYearTopupMethod} yearlyAmount={yearlyAmount} />
        </>
    )
}