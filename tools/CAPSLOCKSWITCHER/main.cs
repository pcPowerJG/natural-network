using System;

using System.Diagnostics;

using System.Windows.Forms;

using System.Runtime.InteropServices;

class InterceptKeys

{

    private const int WH_KEYBOARD_LL = 13;

    private const int WM_KEYDOWN = 0x0100;

    private static LowLevelKeyboardProc _proc = HookCallback;

    private static IntPtr _hookID = IntPtr.Zero;


    public static void Main()

    {

        _hookID = SetHook(_proc);

        Application.Run();

        UnhookWindowsHookEx(_hookID);

    }


    private static IntPtr SetHook(LowLevelKeyboardProc proc)

    {

        using (Process curProcess = Process.GetCurrentProcess())

        using (ProcessModule curModule = curProcess.MainModule)

        {

            return SetWindowsHookEx(WH_KEYBOARD_LL, proc,

                GetModuleHandle(curModule.ModuleName), 0);

        }

    }


    private delegate IntPtr LowLevelKeyboardProc(

        int nCode, IntPtr wParam, IntPtr lParam);

	static int i = 5;
	//static bool temp = false;
	//static bool t1 = false;
    private static IntPtr HookCallback(

        int nCode, IntPtr wParam, IntPtr lParam)

    {

        if (nCode >= 0 /*&& temp*/)

        {

            int vkCode = Marshal.ReadInt32(lParam);
			if ((Keys)vkCode == Keys.Capital && i%6==0){
				//if (n){
				//n = !n;
				//Console.WriteLine((Keys)vkCode);
				SendKeys.Send("(%+)");
				i = 1;
				SendKeys.Send("{CAPSLOCK}");
				//temp = !temp;
				//n = !n;
			}
				
			
			else if ((Keys)vkCode == Keys.Capital && i%6!=0){
				i += 1;
			}
		
        } //else if (nCode >= 0 && temp == false){ temp = !temp; }

        return CallNextHookEx(_hookID, nCode, wParam, lParam);

    }


    [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]

    private static extern IntPtr SetWindowsHookEx(int idHook,

        LowLevelKeyboardProc lpfn, IntPtr hMod, uint dwThreadId);


    [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]

    [return: MarshalAs(UnmanagedType.Bool)]

    private static extern bool UnhookWindowsHookEx(IntPtr hhk);


    [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]

    private static extern IntPtr CallNextHookEx(IntPtr hhk, int nCode,

        IntPtr wParam, IntPtr lParam);


    [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]

    private static extern IntPtr GetModuleHandle(string lpModuleName);

}