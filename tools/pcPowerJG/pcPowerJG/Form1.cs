using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace pcPowerJG
{
    public partial class Form1 : Form
    {
        List<string> words;
        Color specialWordColor = Color.GreenYellow;
        Color backgroundColor = Color.Black;
        Color simpeColor = Color.WhiteSmoke;

        List<RichTextBox> programText;
        int index;

        public Form1()
        {
            InitializeComponent();
            programText = new List<RichTextBox>();
            words = new List<string>();
            words.Add(@"\blet\b");
            words.Add(@"\bin\b");
            words.Add(@"\bfor\b");
            words.Add(@"\bmut\b");
            words.Add(@"\bloop\b");
            words.Add(@"\bfn\b");

            words.Add(@"\bif\b");
            words.Add(@"\belse\b");
            words.Add(@"\b_\b");

            words.Add(@"\breturn\b");
            words.Add(@"\bmatch\b");
            words.Add(@"\buse\b");
            words.Add(@"\bextern\b");
            words.Add(@"\bcrate\b");
            //words.Add(@"\breturn\b");
            richTextBox1.Name = programText.Count.ToString();
            richTextBox1.Click += RichTextBox1_Click;
            richTextBox1.BackColor = backgroundColor;
            richTextBox1.ForeColor = simpeColor;
            programText.Add(richTextBox1);
            this.BackColor = backgroundColor;
        }

        private void RichTextBox1_Click(object sender, EventArgs e)
        {
            //throw new NotImplementedException();
            RichTextBox s = sender as RichTextBox;
            for (int i = 0; i < programText.Count; i++)
                if (programText[i].Name == s.Name)
                {
                    index = i;
                    programText[i].Select(); return;
                }
        }

        private void richTextBox1_KeyUp(object sender, KeyEventArgs e)
        {
            RichTextBox rtb = sender as RichTextBox;
            switch (e.KeyData)
            {
                case Keys.Enter: {
                        rtb.Text = rtb.Text.Remove(rtb.Text.Length - 1);
                        index += 1;
                        if (index >= programText.Count) {
                            RichTextBox nRtb = new RichTextBox();
                            nRtb.Font = rtb.Font;
                            nRtb.Click += RichTextBox1_Click;
                            nRtb.AcceptsTab = true;
                            nRtb.KeyUp += richTextBox1_KeyUp;
                            nRtb.TextChanged += richTextBox1_TextChanged;
                            nRtb.BackColor = programText[programText.Count - 1].BackColor;
                            nRtb.ForeColor = simpeColor;
                            nRtb.Text = "";
                            nRtb.Height = programText[programText.Count - 1].Height;
                            nRtb.Width = programText[programText.Count - 1].Width;
                            nRtb.Name = programText.Count.ToString();
                            nRtb.Location = new Point(
                                rtb.Location.X, rtb.Location.Y + rtb.Height + 1);
                            programText.Add(nRtb);
                            panel1.Controls.Add(nRtb);                            
                        }
                        richTextBox1_TextChanged(sender as RichTextBox, new EventArgs());
                    } break;
                case Keys.Up: {
                        if (index == 0)
                            return;
                        programText = new List<RichTextBox>();
                        programText.Add(richTextBox1);
                        for (int i = 1; i < panel1.Controls.Count; i++)
                        {
                            try
                            {
                                RichTextBox rtb_ = panel1.Controls[i] as RichTextBox;
                                programText.Add(rtb_);
                            }
                            catch { }
                        }
                        if (programText[index].Text == "") {
                            programText.RemoveAt(programText.Count - 1);
                            panel1.Controls.RemoveAt(index);
                        }
                        index -= 1;
                        richTextBox1_TextChanged((sender as RichTextBox), new EventArgs());
                    } break;
                case Keys.Down: {
                        //rtb.Text = rtb.Text.Remove(rtb.Text.Length - 1);                        
                        index += 1;
                        if (index >= programText.Count)
                        {
                            RichTextBox nRtb = new RichTextBox();
                            nRtb.Click += RichTextBox1_Click;
                            nRtb.Font = rtb.Font;
                            nRtb.AcceptsTab = true;
                            nRtb.KeyUp += richTextBox1_KeyUp;
                            nRtb.TextChanged += richTextBox1_TextChanged;
                            nRtb.ForeColor = simpeColor;
                            nRtb.BackColor = programText[programText.Count - 1].BackColor;
                            nRtb.Text = "";
                            nRtb.Height = programText[programText.Count - 1].Height;
                            nRtb.Width = programText[programText.Count - 1].Width;
                            nRtb.Name = programText.Count.ToString();
                            nRtb.Location = new Point(
                                programText[index-1].Location.X, programText[index - 1].Location.Y + programText[index - 1].Height + 1);
                            programText.Add(nRtb);
                            panel1.Controls.Add(nRtb);
                            //panel1.Controls[index].Select();
                        }
                        richTextBox1_TextChanged(sender as RichTextBox, new EventArgs());
                    } break;
                case Keys.F2: {
                        OpenFileDialog op = new OpenFileDialog();
                        op.ShowDialog();

                        StreamReader objReader = new StreamReader(op.FileName);
                        panel1.Controls.Clear();
                        panel1.Refresh();
                        string sLine = "";
                        string textFile = "";
                        index = 0;
                        while (sLine != null)
                        {
                            sLine = objReader.ReadLine();
                            if (sLine != null)
                                textFile += sLine + "\n";
                        }
                        objReader.Close();
                        programText = new List<RichTextBox>();

                        string[] allText = textFile.Split('\n').ToArray();
                        
                        richTextBox1.Text = allText[0];
                        programText.Add(richTextBox1);
                        panel1.Controls.Add(richTextBox1);
                        for (int i = 1; i < allText.Length; i++)
                        {
                            RichTextBox nRtb = new RichTextBox();
                            nRtb.Font = rtb.Font;
                            nRtb.Click += RichTextBox1_Click;
                            nRtb.AcceptsTab = true;
                            nRtb.KeyUp += richTextBox1_KeyUp;
                            nRtb.TextChanged += richTextBox1_TextChanged;
                            nRtb.ForeColor = simpeColor;
                            nRtb.BackColor = programText[programText.Count - 1].BackColor;
                            nRtb.Text = allText[i];
                            nRtb.Height = programText[programText.Count - 1].Height;
                            nRtb.Width = programText[programText.Count - 1].Width;
                            nRtb.Name = programText.Count.ToString();
                            nRtb.Location = new Point(
                                programText[i - 1].Location.X, programText[i - 1].Location.Y + programText[i - 1].Height + 1);
                            programText.Add(nRtb);
                            panel1.Controls.Add(nRtb);
                            richTextBox1_TextChanged(nRtb, new EventArgs());
                        }
                        panel1.Refresh(); this.Refresh();
                    } break;
                case Keys.F5: {
                        SaveFileDialog sfd = new SaveFileDialog();
                        sfd.ShowDialog();

                        string saveText = "";
                        for (int i = 0; i < programText.Count; i++)
                        {
                            saveText += programText[i].Text + "\n";
                        }
                        StreamWriter strWrite = new StreamWriter(sfd.FileName);
                        for (int lk = 0; lk < saveText.Length; lk++)
                        {
                            strWrite.Write(saveText[lk]);
                        }
                        strWrite.Close();
                    } break;
                default:  break;
            }
            panel1.Controls[index].Select();
            panel1.Refresh();
        }

        private void richTextBox1_TextChanged(object sender, EventArgs e)
        {
            int selectInt = (sender as RichTextBox).SelectionStart;
            var currentSelStart = (sender as RichTextBox).SelectionStart;
            var currentSelLength = (sender as RichTextBox).SelectionLength;

            (sender as RichTextBox).SelectAll();
            (sender as RichTextBox).SelectionColor = simpeColor;

            //(sender as RichTextBox).ForeColor = simpeColor;
            foreach (string word in words)
            {
                MatchCollection matches_ = Regex.Matches((sender as RichTextBox).Text, word);

                foreach (var match in matches_.Cast<Match>())
                {
                    (sender as RichTextBox).Select(match.Index, match.Length);
                    (sender as RichTextBox).SelectionColor = specialWordColor;                    
                }

                
            }

            //MatchCollection matches = Regex.Matches(, );
            currentSelStart = -1;
            for (int i = 0; i < (sender as RichTextBox).Text.Length; i++)
            {
                if ((sender as RichTextBox).Text[i] == '\"')
                {
                    currentSelStart = i; break;
                }
            }
            if (currentSelStart == -1)
            {
                (sender as RichTextBox).SelectionStart = selectInt;
                (sender as RichTextBox).SelectionLength = 0;
                return;
            }
                
            int end = 0;
            for (int i = (sender as RichTextBox).Text.Length - 1; i > 0; i--)
            {
                if ((sender as RichTextBox).Text[i] == '\"')
                {
                    end = i + 1; break;
                }
            }

            currentSelLength = end - currentSelStart;

            (sender as RichTextBox).Select(currentSelStart, currentSelLength);
            (sender as RichTextBox).SelectionColor = Color.DarkOrange;
            (sender as RichTextBox).SelectionStart = selectInt;

            (sender as RichTextBox).BackColor = backgroundColor;
            //(sender as RichTextBox).Refresh();
            //panel1.Refresh();
        }
    }
}
