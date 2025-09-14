import ChartMonthAmountApiKey from "./ChartMonthAmount";
import ChartMonthPaymentMethodApiKey from "./ChartMonthPaymentMethod";
import ChartYearAmountApiKey from "./ChartYearAmount";
import ChartYearPaymentMethodApiKey from "./ChartYearPaymentMethod";

interface ChartDashboardMerchanApiKeyProps {
    loadingMonthAmount: boolean;
    loadingMonthPaymentMethod: boolean;
    loadingYearAmount: boolean;
    loadingYearPaymentMethod: boolean

    monthlyAmount: any;
    monthlyPaymentMethod: any;
    yearlyAmount: any;
    yearlyPaymentMethod: any;
}

export default function ChartDashboarMerchantApiKey({
    loadingMonthAmount,
    loadingMonthPaymentMethod,
    loadingYearAmount,
    loadingYearPaymentMethod,
    monthlyAmount,
    monthlyPaymentMethod,
    yearlyAmount,
    yearlyPaymentMethod
}: ChartDashboardMerchanApiKeyProps) {

    return (
        <>
            <ChartMonthPaymentMethodApiKey loadingMonthPaymentMethod={loadingMonthPaymentMethod} monthlyPaymentMethod={monthlyPaymentMethod} />
            <ChartYearPaymentMethodApiKey loadingYearPaymentMethod={loadingYearPaymentMethod} yearlyPaymentMethod={yearlyPaymentMethod} />
            <ChartMonthAmountApiKey loadingMonthAmount={loadingMonthAmount} monthlyAmount={monthlyAmount} />
            <ChartYearAmountApiKey loadingYearAmount={loadingYearAmount} yearlyAmount={yearlyAmount} />
        </>
    )
}