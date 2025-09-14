import { debounce } from "@/helpers/debounce";
import useMerchantStore from "@/store/merchant/merchant";
import { useCallback, useMemo } from "react";


export default function useMonthAmountMerchantApiKey(toast: any) {
    const {
        findMonthAmountByApiKey,
        monthAmount,
        loadingMonthAmount,
        setLoadingMonthAmount,
        setErrorMonthAmount,
    } = useMerchantStore();

    const monthlyAmount = useMemo(() => {
        if (!monthAmount || !Array.isArray(monthAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number, api_key: string) => {
            try {
                setLoadingMonthAmount(true);
                setErrorMonthAmount(null);

                await findMonthAmountByApiKey(toast, year, api_key);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthAmount(false);
            }
        },
        [findMonthAmountByApiKey, setLoadingMonthAmount, setErrorMonthAmount],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 1000);

    return {
        debouncedFetchMonthlyAmount,
        findMonthAmountByApiKey,
        monthAmount,
        loadingMonthAmount,
        setLoadingMonthAmount,
        setErrorMonthAmount,
        monthlyAmount,
        fetchMonthlyAmount
    }
}