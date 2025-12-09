import { debounce } from "@/helpers/debounce";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback, useMemo } from "react";

export default function useMonthAmountWithdraw(toast: any) {
    const {
        findMonthWithdrawAmount,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
    } = useWithdrawStore();

    const monthlyAmount = useMemo(() => {
        if (!monthWithdrawAmount || !Array.isArray(monthWithdrawAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthWithdrawAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthWithdrawAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthWithdrawAmount(true);
                setErrorMonthWithdrawAmount(null);

                await findMonthWithdrawAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthWithdrawAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthWithdrawAmount(false);
            }
        },
        [
            findMonthWithdrawAmount,
            setLoadingMonthWithdrawAmount,
            setErrorMonthWithdrawAmount,
        ],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthWithdrawAmount,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
        monthlyAmount,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmount
    }
}