import { debounce } from "@/helpers/debounce";
import useTopupStore from "@/store/topup/topup";
import { useCallback, useMemo } from "react";

export default function useMonthAmountTopup(toast: any) {
    const {
        findMonthTopupAmount,
        monthTopupAmount,
        loadingMonthTopupAmount,
        setLoadingMonthTopupAmount,
        setErrorMonthTopupAmount,
    } = useTopupStore();

    const monthlyAmount = useMemo(() => {
        if (!monthTopupAmount || !Array.isArray(monthTopupAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTopupAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthTopupAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthTopupAmount(true);
                setErrorMonthTopupAmount(null);

                await findMonthTopupAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthTopupAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthTopupAmount(false);
            }
        },
        [
            findMonthTopupAmount,
            setLoadingMonthTopupAmount,
            setErrorMonthTopupAmount,
        ],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthTopupAmount,
        monthTopupAmount,
        loadingMonthTopupAmount,
        setLoadingMonthTopupAmount,
        setErrorMonthTopupAmount,
        monthlyAmount,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmount
    }
}