namespace MeleeImprooverApp
{
    partial class Form1
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            components = new System.ComponentModel.Container();
            clipLengthTextBox = new TextBox();
            label2 = new Label();
            folderBrowserDialog1 = new FolderBrowserDialog();
            exportButton = new Button();
            dataGridView1 = new DataGridView();
            exportLabel = new Label();
            toolTip1 = new ToolTip(components);
            label5 = new Label();
            importLabel = new Label();
            importButton = new Button();
            searchTextBox = new TextBox();
            searchLabel = new Label();
            refreshButton = new Button();
            exportMenuStrip = new ContextMenuStrip(components);
            button1 = new Button();
            thanksLabel = new Label();
            patreonLabel = new Label();
            aitchPatreonLink = new LinkLabel();
            label1 = new Label();
            linkLabel1 = new LinkLabel();
            numDisplayedFilesLabel = new Label();
            namingStyleBox = new ComboBox();
            label3 = new Label();
            openImportButton = new Button();
            openExportButton = new Button();
            DolphinInfoTip = new Label();
            saveStateButtonComboBox = new ComboBox();
            button2 = new Button();
            DolphinReplayLabel = new Label();
            DolphinConnectButton = new Button();
            ExportP1DolphinButton = new Button();
            ExportP2DolphinButton = new Button();
            notifyIcon1 = new NotifyIcon(components);
            DolphinStatusCircle = new ColoredCircle();
            DolphinP1Label = new Label();
            DolphinP2Label = new Label();
            DolphinToolTip = new ToolTip(components);
            ((System.ComponentModel.ISupportInitialize)dataGridView1).BeginInit();
            SuspendLayout();
            // 
            // clipLengthTextBox
            // 
            clipLengthTextBox.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            clipLengthTextBox.Location = new Point(1272, 104);
            clipLengthTextBox.Margin = new Padding(4, 3, 4, 3);
            clipLengthTextBox.Name = "clipLengthTextBox";
            clipLengthTextBox.Size = new Size(116, 26);
            clipLengthTextBox.TabIndex = 8;
            clipLengthTextBox.TextChanged += clipLengthTextBox_TextChanged;
            clipLengthTextBox.Leave += clipLengthTextBox_Leave;
            // 
            // label2
            // 
            label2.AutoSize = true;
            label2.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            label2.Location = new Point(1161, 107);
            label2.Margin = new Padding(4, 0, 4, 0);
            label2.Name = "label2";
            label2.Size = new Size(89, 20);
            label2.TabIndex = 9;
            label2.Text = "Clip Length";
            // 
            // exportButton
            // 
            exportButton.Font = new Font("Microsoft Sans Serif", 10F);
            exportButton.Location = new Point(595, 48);
            exportButton.Margin = new Padding(4, 3, 4, 3);
            exportButton.Name = "exportButton";
            exportButton.Size = new Size(201, 42);
            exportButton.TabIndex = 10;
            exportButton.Text = "Choose Export Folder";
            exportButton.UseVisualStyleBackColor = true;
            exportButton.Click += exportButton_Click;
            // 
            // dataGridView1
            // 
            dataGridView1.AllowUserToAddRows = false;
            dataGridView1.AllowUserToDeleteRows = false;
            dataGridView1.AllowUserToResizeRows = false;
            dataGridView1.ColumnHeadersHeightSizeMode = DataGridViewColumnHeadersHeightSizeMode.AutoSize;
            dataGridView1.EditMode = DataGridViewEditMode.EditOnEnter;
            dataGridView1.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            dataGridView1.Location = new Point(14, 240);
            dataGridView1.Margin = new Padding(4, 3, 4, 3);
            dataGridView1.Name = "dataGridView1";
            dataGridView1.RowHeadersVisible = false;
            dataGridView1.SelectionMode = DataGridViewSelectionMode.FullRowSelect;
            dataGridView1.Size = new Size(1528, 598);
            dataGridView1.TabIndex = 17;
            dataGridView1.CellBeginEdit += dataGridView1_CellBeginEdit;
            dataGridView1.CellValueChanged += dataGridView1_CellValueChanged;
            dataGridView1.SelectionChanged += dataGridView1_SelectionChanged;
            dataGridView1.MouseDown += dataGridView1_MouseDown;
            // 
            // exportLabel
            // 
            exportLabel.AutoSize = true;
            exportLabel.Font = new Font("Microsoft Sans Serif", 14.25F, FontStyle.Regular, GraphicsUnit.Point, 0);
            exportLabel.Location = new Point(595, 13);
            exportLabel.Margin = new Padding(4, 0, 4, 0);
            exportLabel.MaximumSize = new Size(231, 0);
            exportLabel.Name = "exportLabel";
            exportLabel.Size = new Size(109, 24);
            exportLabel.TabIndex = 20;
            exportLabel.Text = "exportLabel";
            // 
            // label5
            // 
            label5.AutoSize = true;
            label5.Cursor = Cursors.Help;
            label5.Font = new Font("Microsoft Sans Serif", 10F, FontStyle.Bold | FontStyle.Underline);
            label5.Location = new Point(1012, 9);
            label5.Name = "label5";
            label5.Size = new Size(17, 17);
            label5.TabIndex = 50;
            label5.Text = "?";
            DolphinToolTip.SetToolTip(label5, "Connect to an instance of Slippi Dolphin playing a game and export savestates on demand. Make sure only one Dolphin instance is running.");
            // 
            // importLabel
            // 
            importLabel.AutoSize = true;
            importLabel.Font = new Font("Microsoft Sans Serif", 14.25F, FontStyle.Regular, GraphicsUnit.Point, 0);
            importLabel.Location = new Point(334, 13);
            importLabel.Margin = new Padding(4, 0, 4, 0);
            importLabel.MaximumSize = new Size(231, 0);
            importLabel.Name = "importLabel";
            importLabel.Size = new Size(108, 24);
            importLabel.TabIndex = 23;
            importLabel.Text = "importLabel";
            // 
            // importButton
            // 
            importButton.Font = new Font("Microsoft Sans Serif", 10F);
            importButton.Location = new Point(338, 48);
            importButton.Margin = new Padding(4, 3, 4, 3);
            importButton.Name = "importButton";
            importButton.Size = new Size(201, 42);
            importButton.TabIndex = 22;
            importButton.Text = "Choose Replay Folder";
            importButton.UseVisualStyleBackColor = true;
            importButton.Click += importButton_Click;
            // 
            // searchTextBox
            // 
            searchTextBox.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            searchTextBox.Location = new Point(1076, 196);
            searchTextBox.Margin = new Padding(4, 3, 4, 3);
            searchTextBox.Name = "searchTextBox";
            searchTextBox.Size = new Size(212, 26);
            searchTextBox.TabIndex = 24;
            searchTextBox.TextChanged += searchTextBox_TextChanged;
            // 
            // searchLabel
            // 
            searchLabel.AutoSize = true;
            searchLabel.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            searchLabel.Location = new Point(876, 200);
            searchLabel.Margin = new Padding(4, 0, 4, 0);
            searchLabel.Name = "searchLabel";
            searchLabel.Size = new Size(165, 20);
            searchLabel.TabIndex = 25;
            searchLabel.Text = "Search (by File Name)";
            // 
            // refreshButton
            // 
            refreshButton.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            refreshButton.Location = new Point(1330, 196);
            refreshButton.Margin = new Padding(4, 3, 4, 3);
            refreshButton.Name = "refreshButton";
            refreshButton.Size = new Size(212, 30);
            refreshButton.TabIndex = 26;
            refreshButton.Text = "Refresh";
            refreshButton.UseVisualStyleBackColor = true;
            refreshButton.Click += refreshButton_Click;
            // 
            // exportMenuStrip
            // 
            exportMenuStrip.Font = new Font("Segoe UI", 21.75F, FontStyle.Regular, GraphicsUnit.Point, 0);
            exportMenuStrip.Name = "exportMenuStrip";
            exportMenuStrip.Size = new Size(61, 4);
            exportMenuStrip.Click += exportMenuStrip_Click;
            // 
            // button1
            // 
            button1.Font = new Font("Microsoft Sans Serif", 21.75F, FontStyle.Regular, GraphicsUnit.Point, 0);
            button1.Location = new Point(33, 15);
            button1.Margin = new Padding(4, 3, 4, 3);
            button1.Name = "button1";
            button1.Size = new Size(266, 75);
            button1.TabIndex = 27;
            button1.Text = "How to Use";
            button1.UseVisualStyleBackColor = true;
            button1.Click += button1_Click;
            // 
            // thanksLabel
            // 
            thanksLabel.AutoSize = true;
            thanksLabel.Font = new Font("Microsoft Sans Serif", 16F);
            thanksLabel.Location = new Point(16, 925);
            thanksLabel.Margin = new Padding(4, 0, 4, 0);
            thanksLabel.Name = "thanksLabel";
            thanksLabel.Size = new Size(1156, 26);
            thanksLabel.TabIndex = 28;
            thanksLabel.Text = "Special Thanks: Only possible thanks to Aitch for his hard work on the Slippi -> TM-CE backend and RWing! Check out";
            // 
            // patreonLabel
            // 
            patreonLabel.AutoSize = true;
            patreonLabel.Font = new Font("Microsoft Sans Serif", 16F);
            patreonLabel.Location = new Point(15, 875);
            patreonLabel.Margin = new Padding(4, 0, 4, 0);
            patreonLabel.Name = "patreonLabel";
            patreonLabel.Size = new Size(357, 26);
            patreonLabel.TabIndex = 29;
            patreonLabel.Text = "Consider supporting me (Fiction) on";
            // 
            // aitchPatreonLink
            // 
            aitchPatreonLink.AutoSize = true;
            aitchPatreonLink.Font = new Font("Microsoft Sans Serif", 15.75F, FontStyle.Regular, GraphicsUnit.Point, 0);
            aitchPatreonLink.Location = new Point(1360, 927);
            aitchPatreonLink.Margin = new Padding(4, 0, 4, 0);
            aitchPatreonLink.Name = "aitchPatreonLink";
            aitchPatreonLink.Size = new Size(156, 25);
            aitchPatreonLink.TabIndex = 30;
            aitchPatreonLink.TabStop = true;
            aitchPatreonLink.Text = "Aitch's Patreon";
            aitchPatreonLink.LinkClicked += aitchPatreonLink_LinkClicked;
            // 
            // label1
            // 
            label1.AutoSize = true;
            label1.Font = new Font("Microsoft Sans Serif", 16F);
            label1.Location = new Point(519, 875);
            label1.Margin = new Padding(4, 0, 4, 0);
            label1.Name = "label1";
            label1.Size = new Size(459, 26);
            label1.TabIndex = 31;
            label1.Text = "if you want to help support my coding projects!";
            // 
            // linkLabel1
            // 
            linkLabel1.AutoSize = true;
            linkLabel1.Font = new Font("Microsoft Sans Serif", 15.75F, FontStyle.Regular, GraphicsUnit.Point, 0);
            linkLabel1.Location = new Point(424, 876);
            linkLabel1.Margin = new Padding(4, 0, 4, 0);
            linkLabel1.Name = "linkLabel1";
            linkLabel1.Size = new Size(87, 25);
            linkLabel1.TabIndex = 32;
            linkLabel1.TabStop = true;
            linkLabel1.Text = "Patreon";
            // 
            // numDisplayedFilesLabel
            // 
            numDisplayedFilesLabel.AutoSize = true;
            numDisplayedFilesLabel.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            numDisplayedFilesLabel.Location = new Point(14, 201);
            numDisplayedFilesLabel.Margin = new Padding(4, 0, 4, 0);
            numDisplayedFilesLabel.Name = "numDisplayedFilesLabel";
            numDisplayedFilesLabel.Size = new Size(19, 20);
            numDisplayedFilesLabel.TabIndex = 33;
            numDisplayedFilesLabel.Text = "--";
            // 
            // namingStyleBox
            // 
            namingStyleBox.DropDownStyle = ComboBoxStyle.DropDownList;
            namingStyleBox.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            namingStyleBox.FormattingEnabled = true;
            namingStyleBox.Location = new Point(1272, 58);
            namingStyleBox.Margin = new Padding(4, 3, 4, 3);
            namingStyleBox.Name = "namingStyleBox";
            namingStyleBox.Size = new Size(270, 28);
            namingStyleBox.TabIndex = 34;
            namingStyleBox.SelectedIndexChanged += frameComboBox_SelectedIndexChanged;
            // 
            // label3
            // 
            label3.AutoSize = true;
            label3.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            label3.Location = new Point(1146, 61);
            label3.Margin = new Padding(4, 0, 4, 0);
            label3.Name = "label3";
            label3.Size = new Size(102, 20);
            label3.TabIndex = 35;
            label3.Text = "Naming Style";
            // 
            // openImportButton
            // 
            openImportButton.Font = new Font("Microsoft Sans Serif", 10F);
            openImportButton.Location = new Point(338, 99);
            openImportButton.Margin = new Padding(4, 3, 4, 3);
            openImportButton.Name = "openImportButton";
            openImportButton.Size = new Size(201, 42);
            openImportButton.TabIndex = 36;
            openImportButton.Text = "Open Replay Folder";
            openImportButton.UseVisualStyleBackColor = true;
            openImportButton.Click += openImportButton_Click;
            // 
            // openExportButton
            // 
            openExportButton.Font = new Font("Microsoft Sans Serif", 10F);
            openExportButton.Location = new Point(595, 99);
            openExportButton.Margin = new Padding(4, 3, 4, 3);
            openExportButton.Name = "openExportButton";
            openExportButton.Size = new Size(201, 42);
            openExportButton.TabIndex = 37;
            openExportButton.Text = "Open Export Folder";
            openExportButton.UseVisualStyleBackColor = true;
            openExportButton.Click += openExportButton_Click;
            // 
            // DolphinInfoTip
            // 
            DolphinInfoTip.AutoSize = true;
            DolphinInfoTip.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            DolphinInfoTip.Location = new Point(1101, 18);
            DolphinInfoTip.Margin = new Padding(4, 0, 4, 0);
            DolphinInfoTip.Name = "DolphinInfoTip";
            DolphinInfoTip.Size = new Size(133, 20);
            DolphinInfoTip.TabIndex = 39;
            DolphinInfoTip.Text = "Savestate Button";
            // 
            // saveStateButtonComboBox
            // 
            saveStateButtonComboBox.DropDownStyle = ComboBoxStyle.DropDownList;
            saveStateButtonComboBox.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            saveStateButtonComboBox.FormattingEnabled = true;
            saveStateButtonComboBox.Location = new Point(1272, 15);
            saveStateButtonComboBox.Margin = new Padding(4, 3, 4, 3);
            saveStateButtonComboBox.Name = "saveStateButtonComboBox";
            saveStateButtonComboBox.Size = new Size(270, 28);
            saveStateButtonComboBox.TabIndex = 38;
            saveStateButtonComboBox.SelectedIndexChanged += saveStateButtonComboBox_SelectedIndexChanged;
            // 
            // button2
            // 
            button2.Font = new Font("Microsoft Sans Serif", 12F, FontStyle.Regular, GraphicsUnit.Point, 0);
            button2.Location = new Point(33, 102);
            button2.Margin = new Padding(4, 3, 4, 3);
            button2.Name = "button2";
            button2.Size = new Size(266, 39);
            button2.TabIndex = 40;
            button2.Text = "Report Bugs";
            button2.UseVisualStyleBackColor = true;
            button2.Click += button2_Click;
            // 
            // DolphinReplayLabel
            // 
            DolphinReplayLabel.AutoSize = true;
            DolphinReplayLabel.Font = new Font("Microsoft Sans Serif", 14.25F, FontStyle.Regular, GraphicsUnit.Point, 0);
            DolphinReplayLabel.Location = new Point(876, 13);
            DolphinReplayLabel.Margin = new Padding(4, 0, 4, 0);
            DolphinReplayLabel.Name = "DolphinReplayLabel";
            DolphinReplayLabel.Size = new Size(138, 24);
            DolphinReplayLabel.TabIndex = 42;
            DolphinReplayLabel.Text = "Dolphin Replay";
            // 
            // DolphinConnectButton
            // 
            DolphinConnectButton.Font = new Font("Microsoft Sans Serif", 10F);
            DolphinConnectButton.Location = new Point(876, 48);
            DolphinConnectButton.Name = "DolphinConnectButton";
            DolphinConnectButton.Size = new Size(206, 42);
            DolphinConnectButton.TabIndex = 43;
            DolphinConnectButton.Text = "Connect to Dolphin";
            DolphinConnectButton.UseVisualStyleBackColor = true;
            DolphinConnectButton.Click += DolphinConnectButton_Click;
            // 
            // ExportP1DolphinButton
            // 
            ExportP1DolphinButton.Enabled = false;
            ExportP1DolphinButton.Font = new Font("Microsoft Sans Serif", 10F);
            ExportP1DolphinButton.Location = new Point(876, 99);
            ExportP1DolphinButton.Name = "ExportP1DolphinButton";
            ExportP1DolphinButton.Size = new Size(100, 42);
            ExportP1DolphinButton.TabIndex = 44;
            ExportP1DolphinButton.Text = "Export P1";
            ExportP1DolphinButton.UseVisualStyleBackColor = true;
            ExportP1DolphinButton.Click += ExportP1DolphinButton_Click;
            // 
            // ExportP2DolphinButton
            // 
            ExportP2DolphinButton.Enabled = false;
            ExportP2DolphinButton.Font = new Font("Microsoft Sans Serif", 10F);
            ExportP2DolphinButton.Location = new Point(982, 99);
            ExportP2DolphinButton.Name = "ExportP2DolphinButton";
            ExportP2DolphinButton.Size = new Size(100, 42);
            ExportP2DolphinButton.TabIndex = 45;
            ExportP2DolphinButton.Text = "Export P2";
            ExportP2DolphinButton.UseVisualStyleBackColor = true;
            ExportP2DolphinButton.Click += ExportP2DolphinButton_Click;
            // 
            // notifyIcon1
            // 
            notifyIcon1.BalloonTipIcon = ToolTipIcon.Warning;
            notifyIcon1.Text = "notifyIcon1";
            notifyIcon1.Visible = true;
            // 
            // DolphinStatusCircle
            // 
            DolphinStatusCircle.Location = new Point(1052, 58);
            DolphinStatusCircle.Name = "DolphinStatusCircle";
            DolphinStatusCircle.Size = new Size(20, 20);
            DolphinStatusCircle.TabIndex = 46;
            DolphinStatusCircle.Text = "coloredCircle1";
            // 
            // DolphinP1Label
            // 
            DolphinP1Label.AutoSize = true;
            DolphinP1Label.Font = new Font("Microsoft Sans Serif", 10F);
            DolphinP1Label.Location = new Point(876, 144);
            DolphinP1Label.Name = "DolphinP1Label";
            DolphinP1Label.Size = new Size(0, 17);
            DolphinP1Label.TabIndex = 47;
            // 
            // DolphinP2Label
            // 
            DolphinP2Label.AutoSize = true;
            DolphinP2Label.Font = new Font("Microsoft Sans Serif", 10F);
            DolphinP2Label.Location = new Point(982, 144);
            DolphinP2Label.Name = "DolphinP2Label";
            DolphinP2Label.Size = new Size(0, 17);
            DolphinP2Label.TabIndex = 49;
            // 
            // Form1
            // 
            AllowDrop = true;
            AutoScaleDimensions = new SizeF(7F, 15F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(1570, 989);
            Controls.Add(label5);
            Controls.Add(DolphinP2Label);
            Controls.Add(DolphinP1Label);
            Controls.Add(DolphinStatusCircle);
            Controls.Add(ExportP2DolphinButton);
            Controls.Add(ExportP1DolphinButton);
            Controls.Add(DolphinConnectButton);
            Controls.Add(DolphinReplayLabel);
            Controls.Add(button2);
            Controls.Add(DolphinInfoTip);
            Controls.Add(saveStateButtonComboBox);
            Controls.Add(openExportButton);
            Controls.Add(openImportButton);
            Controls.Add(label3);
            Controls.Add(namingStyleBox);
            Controls.Add(numDisplayedFilesLabel);
            Controls.Add(linkLabel1);
            Controls.Add(label1);
            Controls.Add(aitchPatreonLink);
            Controls.Add(patreonLabel);
            Controls.Add(thanksLabel);
            Controls.Add(button1);
            Controls.Add(refreshButton);
            Controls.Add(searchLabel);
            Controls.Add(searchTextBox);
            Controls.Add(importLabel);
            Controls.Add(importButton);
            Controls.Add(exportLabel);
            Controls.Add(dataGridView1);
            Controls.Add(exportButton);
            Controls.Add(label2);
            Controls.Add(clipLengthTextBox);
            Margin = new Padding(4, 3, 4, 3);
            Name = "Form1";
            StartPosition = FormStartPosition.CenterScreen;
            Text = "Fiction's Melee Improover 1.0";
            Load += Form1_Load;
            DragDrop += Form1_DragDrop;
            DragEnter += Form1_DragEnter;
            ((System.ComponentModel.ISupportInitialize)dataGridView1).EndInit();
            ResumeLayout(false);
            PerformLayout();

        }

        #endregion
        private System.Windows.Forms.TextBox clipLengthTextBox;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.FolderBrowserDialog folderBrowserDialog1;
        private System.Windows.Forms.Button exportButton;
        private System.Windows.Forms.DataGridView dataGridView1;
        private System.Windows.Forms.Label exportLabel;
        private System.Windows.Forms.ToolTip toolTip1;
        private System.Windows.Forms.Label importLabel;
        private System.Windows.Forms.Button importButton;
        private System.Windows.Forms.TextBox searchTextBox;
        private System.Windows.Forms.Label searchLabel;
        private System.Windows.Forms.Button refreshButton;
        private System.Windows.Forms.ContextMenuStrip exportMenuStrip;
        private System.Windows.Forms.Button button1;
        private System.Windows.Forms.Label thanksLabel;
        private System.Windows.Forms.Label patreonLabel;
        private System.Windows.Forms.LinkLabel aitchPatreonLink;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.LinkLabel linkLabel1;
        private System.Windows.Forms.Label numDisplayedFilesLabel;
        private System.Windows.Forms.ComboBox namingStyleBox;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.Button openImportButton;
        private System.Windows.Forms.Button openExportButton;
        private System.Windows.Forms.Label DolphinInfoTip;
        private System.Windows.Forms.ComboBox saveStateButtonComboBox;
        private System.Windows.Forms.Button button2;
        private Label DolphinReplayLabel;
        private Button DolphinConnectButton;
        private Button ExportP1DolphinButton;
        private Button ExportP2DolphinButton;
        private NotifyIcon notifyIcon1;
        private ColoredCircle DolphinStatusCircle;
        private Label DolphinP1Label;
        private Label DolphinP2Label;
        private Label label5;
        private ToolTip DolphinToolTip;
    }
}

